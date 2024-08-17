use core::option::Option::None;
use core::option::Option::Some;
use core::option::Option;
use core::marker::Sized;
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
        let ret = Self { pdt_ppn, pdt_vpn, frames: Vec::new() };
        Self::static_map(pdt_vpn, pdt_ppn, PteFlags::P | PteFlags::RW);
        pdt_vpn.get_bytes_array().iter_mut().for_each(|b| *b = 0);
        ret.copy_kernel_space();
        let pde_array = pdt_vpn.get_pde_array();
        let pde = &mut pde_array[1023];
        pde.set_address(pdt_ppn.base_address().0.try_into().unwrap());
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

    pub fn refresh() {
        unsafe {
            asm!("mov cr3, {}", in(reg) Self::pdt_ppn().base_address().0);
        }
    }

    fn copy_kernel_space(&self) {
        let kernel_memory_set = KERNEL_MEMORY_SET.lock();
        let src = kernel_memory_set.page_table.pdt_vpn.get_pde_array();
        let dst = self.pdt_vpn.get_pde_array();
        for idx in 768..=1023 {
            dst[idx] = src[idx];
        }
    }

    pub fn static_map(vpn: VirtPageNum, ppn: PhysPageNum, flag: PteFlags) {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let pde_array = Self::pdt_vpn().get_pde_array();
        let pde = &mut pde_array[index1];
        assert!(pde.flag().contains(PdeFlags::P));
        let address = 0x3ffusize << 22 | index1 << 12;
        let second_vpn: VirtPageNum = VirtAddr(address).into();
        let pte_array = second_vpn.get_pte_array();
        let pte = &mut pte_array[index2];
        assert_eq!(pte.flag().contains(PteFlags::P), false);
        let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), flag);
        *pte = new_entry;
    }

    pub fn map(&self, vpn: VirtPageNum, ppn: PhysPageNum, flag: PteFlags) {
        assert!(self.is_pde_present(vpn));
        self.get_pte_mut(vpn, |pte| {
            assert!(!pte.flag().contains(PteFlags::P), "vpn {:#x} exist pte address {:#x}", vpn.base_address().0, pte.address());
            let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), flag);
            *pte = new_entry;
        });
        assert!(self.is_pte_present(vpn));
        assert_eq!(ppn, self.get_ppn(vpn));
    }

    pub fn map_with_create_pde(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flag: PteFlags) {
        if self.is_pde_present(vpn) {
            self.map(vpn, ppn, flag);
        } else {
            let frame = alloc_phys_frame(1).unwrap();
            let frame_ppn = frame.base_ppn;
            let mut pde_flags = PdeFlags::P | PdeFlags::RW;
            if vpn.base_address().0 < HIGH_ADDRESS_BASE {
                pde_flags |= PdeFlags::US;
            }
            let new_entry = PageDirectoryEntry::new(frame_ppn.base_address().0.try_into().unwrap(), pde_flags);
            self.frames.push(frame);
            self.tmp_map(frame_ppn, |vpn| {
                let bytes_array = unsafe {
                    core::slice::from_raw_parts_mut(vpn.base_address().0 as *mut u8, MEMORY_PAGE_SIZE)
                };
                bytes_array.iter_mut().for_each(|b| *b = 0);
            });
            self.get_pde_mut(vpn, | pde | {
                *pde = new_entry;
            });
            self.map(vpn, ppn, flag);
        }
    }

    pub fn unmap(&self, vpn: VirtPageNum) {
        assert_ne!(vpn, self.pdt_vpn, "can not unmap pdt vpn");
        assert!(self.is_pte_present(vpn));
        self.get_pte_mut(vpn, |pte| {
            *pte = PageTableEntry::empty();
        });
        self.get_pte_ref(vpn, |pte| {
            assert!(!pte.flag().contains(PteFlags::P));
        });
    }

    pub fn set_pte_flag(&self, vpn: VirtPageNum, flag: PteFlags) {
        assert!(self.is_pte_present(vpn));
        self.get_pte_mut(vpn, |pte| {
            pte.set_flag(flag);
        });
    }

    pub fn tmp_map<F: FnOnce(VirtPageNum)>(&self, ppn: PhysPageNum, f: F) {
        let virt_frame_stub = alloc_kernel_virt_frame(1).unwrap();
        self.map(virt_frame_stub.base_vpn, ppn, PteFlags::P | PteFlags::RW);
        // attention!
        // force refresh page table
        PageTable::refresh();
        f(virt_frame_stub.base_vpn);
        self.unmap(virt_frame_stub.base_vpn);
        PageTable::refresh();
    }

    pub fn get_pde_ref<F: FnOnce(&PageDirectoryEntry)>(&self, vpn: VirtPageNum, f: F) {
        let index1 = (vpn.0 >> 10) & 0x3ff;
        let pde_array = unsafe {
            core::slice::from_raw_parts(self.pdt_vpn.base_address().0 as *const PageDirectoryEntry, PTE_SIZE_IN_PAGE)
        };
        f(&pde_array[index1]);
    }

    pub fn get_pde_mut<F: FnOnce(&mut PageDirectoryEntry)>(&self, vpn: VirtPageNum, f: F) {
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let pde_array = unsafe {
            core::slice::from_raw_parts_mut(self.pdt_vpn.base_address().0 as *mut PageDirectoryEntry, PTE_SIZE_IN_PAGE)
        };
        f(&mut pde_array[index1]);
    }

    pub fn is_pde_present(&self, vpn: VirtPageNum) -> bool {
        let mut is_present = false;
        self.get_pde_ref(vpn, |pde| {
            is_present = pde.flag().contains(PdeFlags::P);
        });
        is_present
    }

    pub fn get_pte_page_phys_address(&self, vpn: VirtPageNum) -> usize {
        let mut address = 0;
        self.get_pde_ref(vpn, |pde| {
            address = pde.address() as usize;
        });
        address
    }

    pub fn get_pte_ref<F: FnOnce(&PageTableEntry)>(&self, vpn: VirtPageNum, f: F) {
        assert!(self.is_pde_present(vpn));
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10) & 0x3ff;
        
        if self.pdt_ppn == Self::pdt_ppn() || vpn.base_address().0 >= HIGH_ADDRESS_BASE {
            let address = 0x3ffusize << 22 | index1 << 12;
            let second_vpn: VirtPageNum = VirtAddr(address).into();
            let pte_array = unsafe {
                core::slice::from_raw_parts(second_vpn.base_address().0 as *const PageTableEntry, PTE_SIZE_IN_PAGE)
            };
            f(&pte_array[index2]);
        } else {
            let pte_page_pa = PhysAddr(self.get_pte_page_phys_address(vpn));
            let pte_page_ppn = pte_page_pa.phys_page_num_floor();
            self.tmp_map(pte_page_ppn, |page_vpn| {
                let pte_array = unsafe {
                    core::slice::from_raw_parts(page_vpn.base_address().0 as *const PageTableEntry, PTE_SIZE_IN_PAGE)
                };
                f(&pte_array[index2]);
            });
        }
    }

    pub fn get_pte_mut<F: FnOnce(&mut PageTableEntry)>(&self, vpn: VirtPageNum, f: F) {
        assert!(self.is_pde_present(vpn));
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10) & 0x3ff;
        if self.pdt_ppn == Self::pdt_ppn() || vpn.base_address().0 >= HIGH_ADDRESS_BASE {
            let address = 0x3ffusize << 22 | index1 << 12;
            let second_vpn: VirtPageNum = VirtAddr(address).into();
            let pte_array = unsafe {
                core::slice::from_raw_parts_mut(second_vpn.base_address().0 as *mut PageTableEntry, PTE_SIZE_IN_PAGE)
            };
            f(&mut pte_array[index2]);
        } else {
            let pte_page_pa = PhysAddr(self.get_pte_page_phys_address(vpn));
            let pte_page_ppn = pte_page_pa.phys_page_num_floor();
            self.tmp_map(pte_page_ppn, |page_vpn| {
                let pte_array = unsafe {
                    core::slice::from_raw_parts_mut(page_vpn.base_address().0 as *mut PageTableEntry, PTE_SIZE_IN_PAGE)
                };
                f(&mut pte_array[index2]);
            });
        }
    }

    pub fn is_pte_present(&self, vpn: VirtPageNum) -> bool {
        assert!(self.is_pde_present(vpn));
        let mut is_present = false;
        self.get_pte_ref(vpn, |pte| {
            is_present = pte.flag().contains(PteFlags::P);
        });
        is_present
    }

    pub fn get_ppn(&self, vpn: VirtPageNum) -> PhysPageNum {
        assert!(self.is_pte_present(vpn));
        let mut phys_address = 0;
        self.get_pte_ref(vpn, |pte| {
            phys_address = pte.address() as usize;
        });
        PhysAddr(phys_address).into()
    }

    pub fn get_ref<T: Sized, F: for<'a> FnOnce(&'a T) -> ()>(&self, vpn: VirtPageNum, offset: usize, f: F) {
        if offset + core::mem::size_of::<T>() > MEMORY_PAGE_SIZE {
            return;
        }
        assert!(self.is_pte_present(vpn));
        if self.pdt_ppn == Self::pdt_ppn() || vpn.base_address().0 >= HIGH_ADDRESS_BASE {
            let address = vpn.base_address().0 + offset;
            let value = unsafe { (address as *const T).as_ref() }.unwrap();
            f(value);
        } else {
            let ppn = self.get_ppn(vpn);
            self.tmp_map(ppn, |vpn| {
                let address = vpn.base_address().0 + offset;
                let value = unsafe { (address as *const T).as_ref() }.unwrap();
                f(value);
            })
        }
    }

    pub fn get_mut<T: Sized, F: for<'a> FnOnce(&'a mut T) -> ()>(&self, vpn: VirtPageNum, offset: usize, f: F) {
        if offset + core::mem::size_of::<T>() > MEMORY_PAGE_SIZE {
            return;
        }
        assert!(self.is_pte_present(vpn));
        if self.pdt_ppn == Self::pdt_ppn() || vpn.base_address().0 >= HIGH_ADDRESS_BASE {
            let address = vpn.base_address().0 + offset;
            let value = unsafe { (address as *mut T).as_mut() }.unwrap();
            f(value);
        } else {
            let ppn = self.get_ppn(vpn);
            self.tmp_map(ppn, |vpn| {
                let address = vpn.base_address().0 + offset;
                let value = unsafe { (address as *mut T).as_mut() }.unwrap();
                f(value);
            })
        }
        assert!(self.is_pde_present(vpn));
        assert!(self.is_pte_present(vpn));
    }

    pub fn get_vpn_phys_address(&self, vpn: VPN) -> Option<PhysAddr> {
        if !self.is_pde_present(vpn) {
            return None;
        }
        let mut address: usize = 0;
        self.get_pte_ref(vpn, |pte|{
            address = pte.address() as usize;
        });
        Some(PhysAddr(address))
    }

    pub fn is_vpn_present(&self, vpn: VPN) -> bool {
        return self.is_pde_present(vpn) && self.is_pte_present(vpn)
    }

    pub fn is_vpn_readable(&self, vpn: VPN) -> bool {
        return self.is_pde_present(vpn)
    }

    pub fn is_vpn_writable(&self, vpn: VPN) -> bool {
        if !self.is_pde_present(vpn) {
            return false;
        }
        let mut is_writable = false;
        self.get_pte_ref(vpn, |pte|{
            is_writable = pte.flag().contains(PteFlags::RW);
        });
        return is_writable
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
