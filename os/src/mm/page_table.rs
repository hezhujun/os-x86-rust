use core::option::Option::None;
use core::option::Option::Some;
use core::option::Option;
use core::ops::FnOnce;
use core::convert::From;
use alloc::vec;
use alloc::{sync::Arc, vec::Vec};
use spin::Mutex;
use crate::config::*;
use crate::mm::*;
use crate::{arch::x86::{PageDirectoryEntry, PageTableEntry, PdeFlags, PteFlags}, config::PTE_SIZE_IN_PAGE};

pub struct PageTable {
    pub pdt_ppn: PhysPageNum,
    pub pdt_vpn: VirtPageNum,
    /// 保存申请的 pet page
    frames: Vec<PhysFrameStub>,
}

impl PageTable {
    pub fn new(pdt_ppn: PhysPageNum, pdt_vpn: VirtPageNum) -> Self {
        let mut ret = Self { pdt_ppn, pdt_vpn, frames: Vec::new() };
        ret.map(pdt_vpn, pdt_ppn, PteFlags::P | PteFlags::RW);
        pdt_vpn.get_bytes_array().iter_mut().for_each(|b| *b = 0);
        ret.copy_kernel_space();
        ret
    }

    pub fn from_exists(pdt_ppn: PhysPageNum, pdt_vpn: VirtPageNum) -> Self {
        Self { pdt_ppn, pdt_vpn, frames: Vec::new() }
    }
}

impl PageTable {
    pub fn pdt_vpn() -> VirtPageNum {
        VirtAddr(PDT_VIRT_ADDRESS).into()
    }

    pub fn pdt_ppn() -> PhysPageNum {
        let pde_array = Self::pdt_vpn().get_pde_array();
        PhysAddr(pde_array[0x3ff].address().try_into().unwrap()).into()
    }

    fn copy_kernel_space(&self) {
        let kernel_memory_set = KERNEL_MEMORY_SET.lock();
        let src = kernel_memory_set.page_table.pdt_vpn.get_pde_array();
        let dst = self.pdt_vpn.get_pde_array();
        for idx in 768..1023 {
            dst[idx] = src[idx];
        }
    }

    /// 映射 vpn 到 ppn
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flag: PteFlags) {
        self.find_pte(vpn, true, |pte| {
            assert_eq!(pte.flag().contains(PteFlags::P), false);
            let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), flag);
            *pte = new_entry;
        });
    }

    /// 取消映射 vpn 到 ppn
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let ret = self.find_pte(vpn, false, |pte| {
            assert!(pte.flag().contains(PteFlags::P));
            *pte = PageTableEntry::empty();
        });
        assert!(ret);
    }

    pub fn tmp_map<F: FnOnce(VirtPageNum)>(&mut self, ppn: PhysPageNum, f: F) {
        let virt_frame_stub = alloc_kernel_virt_frame(1).unwrap();
        self.map(virt_frame_stub.base_vpn, ppn, PteFlags::P | PteFlags::RW);
        unsafe {
            asm!("mov cr3, {}", in(reg) KERNEL_PDT_PHYS_ADDRESS);
        }
        f(virt_frame_stub.base_vpn);
        self.unmap(virt_frame_stub.base_vpn);
        unsafe {
            asm!("mov cr3, {}", in(reg) KERNEL_PDT_PHYS_ADDRESS);
        }
    }

    pub fn get_page_bytes_array<F: FnOnce(&mut [u8])>(&mut self, vpn: VirtPageNum, f: F) {
        if self.pdt_ppn == Self::pdt_ppn() || vpn.base_address().0 >= HIGH_ADDRESS_BASE {
            let bytes_array = vpn.get_bytes_array();
            f(bytes_array);
        } else {
            let index2 = vpn.0 & 0x3ff;
            let index1 = (vpn.0 >> 10)& 0x3ff;
            let pde_array = self.pdt_vpn.get_pde_array();
            let pde = &mut pde_array[index1];
            assert!(pde.flag().contains(PdeFlags::P));

            let page_pa = PhysAddr(pde.address().try_into().unwrap());
            let page_ppn = page_pa.phys_page_num_floor();
            self.tmp_map(page_ppn, |page_vpn| {
                let bytes_array = page_vpn.get_bytes_array();
                f(bytes_array);
            });
        }
    }

    fn find_pte<F: FnOnce(&mut PageTableEntry)>(&mut self, vpn: VirtPageNum, is_create_pde: bool, f: F) -> bool {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let pde_array = self.pdt_vpn.get_pde_array();
        let pde = &mut pde_array[index1];
        if !pde.flag().contains(PdeFlags::P) {
            if is_create_pde {
                let frame = alloc_phys_frame(1).unwrap();
                let new_entry = PageDirectoryEntry::new(frame.base_ppn.base_address().0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
                self.frames.push(frame);
                *pde = new_entry;
            } else {
                return false;
            }
        }
        if self.pdt_ppn == Self::pdt_ppn() || vpn.base_address().0 >= HIGH_ADDRESS_BASE {
            let address = 0x3ffusize << 22 | index1 << 12;
            let second_vpn: VirtPageNum = VirtAddr(address).into();
            let pte_array = second_vpn.get_pte_array();
            f(&mut pte_array[index2]);
        } else {
            let page_pa = PhysAddr(pde.address().try_into().unwrap());
            let page_ppn = page_pa.phys_page_num_floor();
            self.tmp_map(page_ppn, |page_vpn| {
                let pte_array = page_vpn.get_pte_array();
                f(&mut pte_array[index2]);
            });
        }
        true
    }
}

impl VirtPageNum {
    pub fn get_pde_array(&self) -> &'static mut [PageDirectoryEntry] {
        let pa: VirtAddr = self.base_address();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageDirectoryEntry, PTE_SIZE_IN_PAGE)
        }
    }

    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: VirtAddr = self.base_address();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PTE_SIZE_IN_PAGE)
        }
    }
}

impl From<PageDirectoryEntry> for VirtPageNum {
    fn from(value: PageDirectoryEntry) -> Self {
        VirtAddr(value.address() as usize).into()
    }
}

impl From<PageTableEntry> for VirtPageNum {
    fn from(value: PageTableEntry) -> Self {
        VirtAddr(value.address() as usize).into()
    }
}
