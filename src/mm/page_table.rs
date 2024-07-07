use alloc::vec;
use alloc::{sync::Arc, vec::Vec};
use spin::Mutex;
use super::{alloc_frame, FrameTracker, PhysAddr, PhysPageNum, VirtPageNum};
use crate::mm::KERNEL_PDT_ADDRESS;
use crate::{arch::x86::{PageDirectoryEntry, PageTableEntry, PdeFlags, PteFlags}, config::PTE_SIZE_IN_PAGE};

pub struct PageTable {
    root: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new(root: PhysPageNum, frames: Vec<FrameTracker>) -> Self {
        Self { root, frames }
    }

    pub fn from(root: PhysPageNum) -> Self {
        Self { root, frames: Vec::new() }
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

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let pde_array = self.root.get_pde_arrray();
        if !pde_array[index1].flag().contains(PdeFlags::P) {
            return None;
        }
        let second_ppn: PhysPageNum = pde_array[index1].into();
        let pte_array = second_ppn.get_pte_arrray();
        Some(&mut pte_array[index2])
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> &mut PageTableEntry {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let pde_array = self.root.get_pde_arrray();
        if !pde_array[index1].flag().contains(PdeFlags::P) {
            self.create_entry(&mut pde_array[index1]);
        }
        let second_ppn: PhysPageNum = pde_array[index1].into();
        let pte_array = second_ppn.get_pte_arrray();
        &mut pte_array[index2]
    }
}

impl PhysPageNum {
    pub fn get_pde_arrray(&self) -> &'static mut [PageDirectoryEntry] {
        let pa: PhysAddr = self.base_address();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageDirectoryEntry, PTE_SIZE_IN_PAGE)
        }
    }

    pub fn get_pte_arrray(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.base_address();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PTE_SIZE_IN_PAGE)
        }
    }
}

impl From<PageDirectoryEntry> for PhysPageNum {
    fn from(value: PageDirectoryEntry) -> Self {
        PhysAddr(value.address() as usize).into()
    }
}

impl From<PageTableEntry> for PhysPageNum {
    fn from(value: PageTableEntry) -> Self {
        PhysAddr(value.address() as usize).into()
    }
}

lazy_static! {
    pub static ref KERNEL_PAGE_TABLE: Arc<Mutex<PageTable>> = {
        Arc::new(Mutex::new(PageTable::from(PhysAddr(KERNEL_PDT_ADDRESS).into())))
    };
}
