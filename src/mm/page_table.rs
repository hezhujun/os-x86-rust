use alloc::vec;
use alloc::{sync::Arc, vec::Vec};
use spin::Mutex;
use super::{alloc_frame, FrameTracker, PhysAddr, PhysPageNum, VirtPageNum};
use crate::{arch::x86::{PageDirectoryEntry, PageTableEntry, PdeFlags, PteFlags}, config::PTE_SIZE_IN_PAGE};

pub struct PageTable {
    root: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new(root: PhysPageNum, frames: Vec<FrameTracker>) -> Self {
        Self { root, frames }
    }

    pub fn kernel_page_table() -> Self {
        // root + 1M / 4KB = 1 + 0x100000 / 0x1000 = 1 + 0x100 = 1 + 256
        let root_frame = alloc_frame(1).unwrap();
        let root = root_frame.base_ppn;
        let pte_frames = alloc_frame(256).unwrap();
        let pde_array = root.get_pde_arrray();
        for idx in 0..0x100 {
            let entry = PageDirectoryEntry::new(pte_frames.base_ppn.base_address().0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW | PdeFlags::US);
            pde_array[idx] = entry
        }
        let frames = vec![root_frame, pte_frames];
        Self { root, frames }
    }

    pub fn map_page(&mut self, virt_page_num: VirtPageNum, phys_page_num: PhysPageNum, flag: PteFlags) {
        let index2 = virt_page_num.0 & 0x3ff;
        let index1 = (virt_page_num.0 >> 10)& 0x3ff;
        let pde_array = self.root.get_pde_arrray();
        if !pde_array[index1].flag().contains(PdeFlags::P) {
            self.create_entry(&mut pde_array[index1]);
        }
        let second_ppn: PhysPageNum = pde_array[index1].into();
        let pte_array = second_ppn.get_pte_arrray();
        assert_eq!(pte_array[index2].flag().contains(PteFlags::P), false);
        let new_entry = PageTableEntry::new(phys_page_num.base_address().0.try_into().unwrap(), flag);
        pte_array[index2] = new_entry;
    }

    fn create_entry(&mut self, entry: &mut PageDirectoryEntry) {
        let frame = alloc_frame(1).unwrap();
        let new_entry = PageDirectoryEntry::new(frame.base_ppn.base_address().0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
        self.frames.push(frame);
        *entry = new_entry;
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
        Arc::new(Mutex::new(PageTable::kernel_page_table()))
    };
}
