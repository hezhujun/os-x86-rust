use core::option::Option::None;
use core::option::Option::Some;
use core::option::Option;
use core::convert::From;
use alloc::vec;
use alloc::{sync::Arc, vec::Vec};
use spin::Mutex;
use super::{alloc_frame, FrameTracker, PhysAddr, PhysPageNum, VirtPageNum};
use crate::config::PAGE_TABLE_VIRT_ADDRESS;
use crate::mm::*;
use crate::{arch::x86::{PageDirectoryEntry, PageTableEntry, PdeFlags, PteFlags}, config::PTE_SIZE_IN_PAGE};

pub struct PageTable {
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn root_vpn() -> VirtPageNum {
        VirtAddr(PAGE_TABLE_VIRT_ADDRESS).into()
    }

    pub fn root_ppn() -> PhysPageNum {
        let pde_array = Self::root_vpn().get_pde_arrray();
        PhysAddr(pde_array[0x3ff].address().try_into().unwrap()).into()
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flag: PteFlags) {
        let pte = self.find_pte_create(vpn);
        assert_eq!(pte.flag().contains(PteFlags::P), false);
        let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), flag);
        *pte = new_entry;
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        if let Some(pte) = self.find_pte(vpn) {
            assert!(pte.flag().contains(PteFlags::P));
            *pte = PageTableEntry::empty();
        } else {
            assert!(false)
        }
    }

    fn create_entry(&mut self, entry: &mut PageDirectoryEntry) {
        let frame = alloc_frame(1).unwrap();
        let new_entry = PageDirectoryEntry::new(frame.base_ppn.base_address().0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
        self.frames.push(frame);
        *entry = new_entry;
    }

    fn get_pte_arrray(&self, idx1: usize) -> &'static mut [PageTableEntry] {
        let idx1 = idx1 & 0x3ff;
        let address = 0x3ffusize << 22 | idx1 << 12;
        let second_vpn: VirtPageNum = VirtAddr(address).into();
        second_vpn.get_pte_arrray()
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let root = Self::root_vpn();
        let pde_array = root.get_pde_arrray();
        if !pde_array[index1].flag().contains(PdeFlags::P) {
            return None;
        }
        let pte_array = self.get_pte_arrray(index1);
        Some(&mut pte_array[index2])
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> &mut PageTableEntry {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let root = Self::root_vpn();
        let pde_array = root.get_pde_arrray();
        if !pde_array[index1].flag().contains(PdeFlags::P) {
            self.create_entry(&mut pde_array[index1]);
        }
        let pte_array = self.get_pte_arrray(index1);
        &mut pte_array[index2]
    }
}

impl VirtPageNum {
    pub fn get_pde_arrray(&self) -> &'static mut [PageDirectoryEntry] {
        let pa: VirtAddr = self.base_address();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageDirectoryEntry, PTE_SIZE_IN_PAGE)
        }
    }

    pub fn get_pte_arrray(&self) -> &'static mut [PageTableEntry] {
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

lazy_static! {
    pub static ref KERNEL_PAGE_TABLE: Arc<Mutex<PageTable>> = {
        Arc::new(Mutex::new(PageTable::new()))
    };
}
