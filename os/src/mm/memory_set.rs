use core::convert::From;
use alloc::{collections::BTreeMap, sync::Arc};
use alloc::vec;
use alloc::vec::Vec;
use bitflags::bitflags;
use spin::Mutex;

use crate::arch::x86::PteFlags;
use crate::config::{KERNEL_PDT_PHYS_ADDRESS, MEMORY_PAGE_SIZE};
use crate::mm::{alloc_kernel_virt_frame, PhysAddr, VirtAddr};
use crate::utils::*;

use super::VirtFrameStub;
use super::{alloc_phys_frame, page_table::{self, PageTable}, PhysFrameStub, PhysPageNum, VPNRange, VirtPageNum};

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1;
        const W = 1 << 1;
        const X = 1 << 2;
        const U = 1 << 3;
    }
}

impl From<MapPermission> for PteFlags {
    fn from(value: MapPermission) -> Self {
        let mut ret = PteFlags::P;
        if value.contains(MapPermission::W) {
            ret |= PteFlags::RW;
        }
        if value.contains(MapPermission::U) {
            ret |= PteFlags::US;
        }
        ret
    }
}

pub struct MapArea {
    pub vpn_range: VPNRange,
    pub map_perm: MapPermission,
    data_frames: BTreeMap<VirtPageNum, Arc<PhysFrameStub>>,
}

impl MapArea {
    pub fn new(vpn_range: VPNRange, map_perm: MapPermission) -> Self {
        Self { vpn_range, map_perm, data_frames: BTreeMap::new()}
    }

    pub fn copy(&self) -> Self {
        let vpn_range = self.vpn_range.clone();
        let map_perm = self.map_perm;
        MapArea { vpn_range, map_perm, data_frames: self.data_frames.clone() }
    }

    /// 映射 vpn 到 ppn，并清理 vpn 页内容
    /// 返回内容：是否有修复页表
    pub fn map_if_need(&mut self, page_table: &mut PageTable) -> bool {
        let mut is_modified = false;
        for vpn in self.vpn_range.clone() {
            if !page_table.is_vpn_present(vpn) {
                self.map_once(page_table, vpn);
                is_modified = true
            }
        }
        is_modified
    }

    /// 取消映射 vpn 到 ppn
    pub fn unmap(&mut self, page_table: &PageTable) {
        for vpn in self.vpn_range.clone() {
            if page_table.is_vpn_present(vpn) {
                self.unmap_once(page_table, vpn);
            }
        }
    }

    pub fn change_perm(&self, map_perm: MapPermission, page_table: &PageTable) {
        for vpn in self.vpn_range.clone() {
            page_table.set_pte_flag(vpn, map_perm.into());
        }
    }

    pub fn copy_if_need(&mut self, page_table: &mut PageTable) -> bool {
        let mut is_modified = false;
        if self.map_perm.contains(MapPermission::W) {
            for vpn in self.vpn_range.clone() {
                assert!(page_table.is_vpn_present(vpn));
                if !page_table.is_vpn_writable(vpn) {
                    let mut is_need_remap = false;
                    if let Some(frame_stub) = self.data_frames.get(&vpn) {
                        if Arc::strong_count(frame_stub) == 1 {
                            page_table.set_pte_flag(vpn, self.map_perm.into());
                        } else {
                            is_need_remap = true;
                        }
                    } else {
                        assert!(false);
                    }
                    if is_need_remap {
                        let frame = alloc_phys_frame(1).unwrap();
                        let ppn = frame.base_ppn;
                        page_table.remap_for_fork_process(vpn, ppn, self.map_perm.into());
                        self.data_frames.insert(vpn, Arc::new(frame));
                    }
                    is_modified = true;
                }
            }
        }
        is_modified
    }

    fn map_once(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let frame = alloc_phys_frame(1).unwrap();
        let ppn: PhysPageNum = frame.base_ppn;
        self.data_frames.insert(vpn, Arc::new(frame));
        page_table.map_with_create_pde(vpn, ppn, self.map_perm.into());
        assert!(page_table.is_pte_present(vpn));
        page_table.get_mut::<[u8; MEMORY_PAGE_SIZE], _>(vpn, 0, |bytes_array| {
            bytes_array.iter_mut().for_each(|b| * b = 0);
        });
        assert!(page_table.is_pte_present(vpn));
    }

    fn unmap_once(&mut self, page_table: &PageTable, vpn: VirtPageNum) {
        self.data_frames.remove(&vpn);
        page_table.unmap(vpn);
    }

    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        let mut start: usize = 0;
        let len = data.len();
        for idx in self.vpn_range.start.0..self.vpn_range.end.0 {
            let vpn = VirtPageNum(idx);
            let src = &data[start..len.min(start + MEMORY_PAGE_SIZE)];
            page_table.get_mut::<[u8; MEMORY_PAGE_SIZE], _>(vpn, 0, | bytes_array | {
                let dst = &mut bytes_array[..src.len()];
                dst.copy_from_slice(src);
                start += MEMORY_PAGE_SIZE;
            });
            if start >= len {
                break;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ProgramHeader {
    pub virtual_addr: usize,
    pub mem_size: usize,
    pub file_offset: usize,
    pub file_size: usize,
    pub flags: xmas_elf::program::Flags,
}

pub struct MemorySet {
    pdt_pstub: PhysFrameStub,
    pdt_vstub: VirtFrameStub,
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
    user_stack_base: usize,
    pub program_headers: Option<Vec<ProgramHeader>>,
}

impl Drop for MemorySet {
    fn drop(&mut self) {
        self.page_table.unmap(self.pdt_vstub.base_vpn);
    }
}

impl MemorySet {
    pub fn new_kernel_memory_set() -> Self {
        // 创建 page_table
        let pdt_pstub = alloc_phys_frame(1).unwrap();
        let pdt_ppn = pdt_pstub.base_ppn;
        let pdt_vstub = alloc_kernel_virt_frame(1).unwrap();
        let pdt_vpn = pdt_vstub.base_vpn;
        let page_table = PageTable::new(pdt_ppn, pdt_vpn);
        MemorySet { pdt_pstub, pdt_vstub, page_table, areas: Vec::new(), user_stack_base: 0, program_headers: None }
    }

    /// return MemorySet and entry point
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize) {
        // 创建 page_table
        let pdt_pstub = alloc_phys_frame(1).unwrap();
        let pdt_ppn = pdt_pstub.base_ppn;
        let pdt_vstub = alloc_kernel_virt_frame(1).unwrap();
        let pdt_vpn = pdt_vstub.base_vpn;
        let page_table = PageTable::new(pdt_ppn, pdt_vpn);

        // program headers of elf, with U flag
        let mut program_headers = Vec::new();
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count: u16 = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let program_header = ProgramHeader {
                    virtual_addr: ph.virtual_addr() as usize,
                    mem_size: ph.mem_size() as usize,
                    file_offset: ph.offset() as usize,
                    file_size: ph.file_size() as usize,
                    flags: ph.flags(),
                };
                program_headers.push(program_header);
                let end_va = VirtAddr((ph.virtual_addr() + ph.mem_size()) as usize);
                max_end_vpn = end_va.virt_page_num_ceil();
            }
        }
        let max_end_va: VirtAddr = max_end_vpn.base_address();
        let mut user_stack_base: usize = max_end_va.0;
        // guard page
        user_stack_base += MEMORY_PAGE_SIZE;

        let areas = Self::generate_map_area(&program_headers);
        let memory_set = MemorySet { pdt_pstub, pdt_vstub, page_table, areas, user_stack_base: user_stack_base, program_headers: Some(program_headers) };

        (memory_set, elf.header.pt2.entry_point() as usize)
    }

    pub fn reset_from_elf(&mut self, elf_data: &[u8]) -> usize {
        for area in &mut self.areas {
            area.unmap(&mut self.page_table);
        }

        // program headers of elf, with U flag
        let mut program_headers = Vec::new();
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count: u16 = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let program_header = ProgramHeader {
                    virtual_addr: ph.virtual_addr() as usize,
                    mem_size: ph.mem_size() as usize,
                    file_offset: ph.offset() as usize,
                    file_size: ph.file_size() as usize,
                    flags: ph.flags(),
                };
                program_headers.push(program_header);
                let end_va = VirtAddr((ph.virtual_addr() + ph.mem_size()) as usize);
                max_end_vpn = end_va.virt_page_num_ceil();
            }
        }
        let max_end_va: VirtAddr = max_end_vpn.base_address();
        let mut user_stack_base: usize = max_end_va.0;
        // guard page
        user_stack_base += MEMORY_PAGE_SIZE;

        let areas = Self::generate_map_area(&program_headers);
        self.areas = areas;
        self.user_stack_base = user_stack_base;
        self.program_headers = Some(program_headers);

        elf.header.pt2.entry_point() as usize
    }

    pub fn copy(&self) -> Self {
        // 创建 page_table
        let pdt_pstub = alloc_phys_frame(1).unwrap();
        let pdt_ppn = pdt_pstub.base_ppn;
        let pdt_vstub = alloc_kernel_virt_frame(1).unwrap();
        let pdt_vpn = pdt_vstub.base_vpn;
        let page_table = self.page_table.copy(pdt_ppn, pdt_vpn);

        // 设置当前进程和新进程用户空间的内存只读
        for area in &self.areas {
            if area.map_perm.contains(MapPermission::W) {
                let mut map_perm: MapPermission = area.map_perm;
                map_perm.remove(MapPermission::W);
                area.change_perm(map_perm, &page_table);
            }
        }

        let mut new_areas = Vec::new();
        for area in &self.areas {
            new_areas.push(area.copy());
        }

        MemorySet { 
            pdt_pstub, 
            pdt_vstub, 
            page_table, 
            areas: new_areas, 
            user_stack_base: self.user_stack_base, 
            program_headers: self.program_headers.clone(),
        }
    }

    fn generate_map_area(program_headers: &Vec<ProgramHeader>) -> Vec<MapArea> {
        let mut areas: Vec<MapArea> = Vec::new();
        // asume program_headers are sorted
        for ph in program_headers {
            let start_va = VirtAddr(ph.virtual_addr);
            let end_va = VirtAddr(ph.virtual_addr + ph.mem_size);
            let mut start_vpn = start_va.virt_page_num_floor();
            let end_vpn = end_va.virt_page_num_ceil();
            let mut map_perm = MapPermission::U;
            if ph.flags.is_read() {
                map_perm |= MapPermission::R;
            }
            if ph.flags.is_write() {
                map_perm |= MapPermission::W;
            }
            if ph.flags.is_execute() {
                map_perm |= MapPermission::X;
            }
            let area = if let Some(mut area) = areas.pop() {
                if area.vpn_range.end > start_vpn {
                    area.map_perm = area.map_perm.union(map_perm);
                    start_vpn = area.vpn_range.end;
                    if start_vpn < end_vpn {
                        areas.push(area);
                        MapArea::new(start_vpn..end_vpn, map_perm)
                    } else {
                        area
                    }
                } else {
                    areas.push(area);
                    MapArea::new(start_vpn..end_vpn, map_perm)
                }
            } else {
                MapArea::new(start_vpn..end_vpn, map_perm)
            };
            areas.push(area);
        }
        areas
    }
}

lazy_static! {
    pub static ref KERNEL_MEMORY_SET: Arc<Mutex<MemorySet>> = {
        let pdt_pa = PhysAddr(KERNEL_PDT_PHYS_ADDRESS);
        let pdt_ppn = pdt_pa.phys_page_num_floor();
        let pdt_pstub = PhysFrameStub { base_ppn: pdt_ppn, len: 1 };
        let pdt_vstub = alloc_kernel_virt_frame(1).unwrap();
        let pdt_vpn = pdt_vstub.base_vpn;
        PageTable::static_map(pdt_vpn, pdt_ppn, PteFlags::P | PteFlags::RW);
        let page_table = PageTable::from_exists(pdt_ppn, pdt_vpn);
        // 内核的 user_stack_base 没有作用
        Arc::new(Mutex::new(MemorySet {pdt_pstub,pdt_vstub,page_table,areas:Vec::new(),user_stack_base:0, program_headers: None }))
    };
}
