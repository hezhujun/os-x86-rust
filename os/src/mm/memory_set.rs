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
    vpn_range: VPNRange,
    vpn_stub: Option<VirtFrameStub>,
    data_frames: BTreeMap<VirtPageNum, PhysFrameStub>,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(vpn_range: VPNRange, vpn_stub: Option<VirtFrameStub>, map_perm: MapPermission) -> Self {
        Self { vpn_range, vpn_stub, data_frames: BTreeMap::new(), map_perm }
    }

    /// 映射 vpn 到 ppn，并清理 vpn 页内容
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range.start.0..self.vpn_range.end.0 {
            self.map_once(page_table, VirtPageNum(vpn));
        }
    }

    /// 取消映射 vpn 到 ppn
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range.start.0..self.vpn_range.end.0 {
            self.unmap_once(page_table, VirtPageNum(vpn));
        }
    }

    fn map_once(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let frame = alloc_phys_frame(1).unwrap();
        let ppn: PhysPageNum = frame.base_ppn;
        self.data_frames.insert(vpn, frame);
        page_table.map_with_create_pde(vpn, ppn, self.map_perm.into());
        assert!(page_table.is_pte_present(vpn));
        page_table.get_mut::<[u8; MEMORY_PAGE_SIZE], _>(vpn, 0, |bytes_array| {
            bytes_array.iter_mut().for_each(|b| * b = 0);
        });
        assert!(page_table.is_pte_present(vpn));
    }

    fn unmap_once(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
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

pub struct MemorySet {
    pdt_pstub: PhysFrameStub,
    pdt_vstub: VirtFrameStub,
    pub page_table: PageTable,
    areas: Vec<MapArea>,
    user_stack_base: usize,
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
        MemorySet { pdt_pstub, pdt_vstub, page_table, areas: Vec::new(), user_stack_base: 0 }
    }

    /// return MemorySet and entry point
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize) {
        // 创建 page_table
        let pdt_pstub = alloc_phys_frame(1).unwrap();
        let pdt_ppn = pdt_pstub.base_ppn;
        let pdt_vstub = alloc_kernel_virt_frame(1).unwrap();
        let pdt_vpn = pdt_vstub.base_vpn;
        let page_table = PageTable::new(pdt_ppn, pdt_vpn);

        let mut memory_set = MemorySet { pdt_pstub, pdt_vstub, page_table, areas: Vec::new(), user_stack_base: 0 };

        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count: u16 = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va = VirtAddr(ph.virtual_addr() as usize);
                let end_va = VirtAddr((ph.virtual_addr() + ph.mem_size()) as usize);
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() { map_perm |= MapPermission::R; }
                if ph_flags.is_write() { map_perm |= MapPermission::W; }
                if ph_flags.is_execute() { map_perm |= MapPermission::X; }
                let map_area = MapArea::new(
                    start_va.virt_page_num_floor()..end_va.virt_page_num_ceil(),
                    None,
                    map_perm
                );
                max_end_vpn = map_area.vpn_range.end;
                memory_set.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize])
                );
            }
        }
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.base_address();
        let mut user_stack_base: usize = max_end_va.0;
        // guard page
        user_stack_base += MEMORY_PAGE_SIZE;
        memory_set.user_stack_base = user_stack_base;
        (memory_set, elf.header.pt2.entry_point() as usize)
    }

    pub fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(map_area);
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
        Arc::new(Mutex::new(MemorySet { pdt_pstub, pdt_vstub, page_table, areas: Vec::new(), user_stack_base: 0 }))
    };
}
