use core::option::Option;
use alloc::sync::Arc;
use spin::Mutex;

use super::{PhysPageNum, MEMORY_INFO};

pub trait FrameAllocator {
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn alloc_contiguous_pages(&mut self, count: usize) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

/// 0x100000000 >> 12
const PAGE_MAX_NUMBER_IN_4G: usize = 0x100000;
const PAGE_BITMAP_LEN_IN_4G: usize = PAGE_MAX_NUMBER_IN_4G / 8;

struct Bitmap<const N: usize> {
    map: [u8; N]
}

impl<const N: usize> Bitmap<{ N }> {
    fn new(map: [u8; N]) -> Self {
        Self { map }
    }

    fn set(&mut self, idx: usize, value: bool) {
        assert!(self.map.len() * 8 > idx);
        let index = idx / 8;
        let offset = idx % 8;
        let bit_mask = 1u8 << offset;
        if value {
            self.map[index] |= bit_mask;
        } else {
            self.map[index] &= !bit_mask;
        }
    }

    fn get(&self, idx: usize) -> bool {
        assert!(self.map.len() * 8 > idx);
        let index = idx / 8;
        let offset = idx % 8;
        let bit_mask = 1u8 << offset;
        self.map[index] & bit_mask != 0
    }
}

pub struct SimpleAllocator {
    page_map: Bitmap<PAGE_BITMAP_LEN_IN_4G>,
    begin: usize,
    end: usize,
    current: usize,
}

impl SimpleAllocator {
    fn new(begin: PhysPageNum, end: PhysPageNum) -> Self {
        Self { page_map: Bitmap::new([0; PAGE_BITMAP_LEN_IN_4G]), begin: begin.0, end: end.0, current: begin.0 }
    }
}

impl FrameAllocator for SimpleAllocator {
    fn alloc(&mut self) -> Option<PhysPageNum> {
        for idx in self.begin..self.current {
            if !self.page_map.get(idx) {
                self.page_map.set(idx, true);
                return Some(PhysPageNum(idx));
            }
        }
        if self.current < self.end {
            self.page_map.set(self.current, true);
            self.current += 1;
            return Some(PhysPageNum(self.current - 1));
        }
        None
    }

    fn alloc_contiguous_pages(&mut self, count: usize) -> Option<PhysPageNum> {
        if self.end - self.current >= count {
            for idx in 0..count {
                self.page_map.set(self.current + idx, true);
            }
            self.current += count;
            return Some(PhysPageNum(self.current - count));
        }
        let mut idx = self.begin;
        while idx < self.end {
            let mut free_idx:usize = 0;
            while free_idx < count && !self.page_map.get(idx + free_idx) {
                free_idx += 1;
            }
            if free_idx == count {
                for i in 0..count {
                    self.page_map.set(idx + i, true);
                }
                return Some(PhysPageNum(idx));
            }
            idx += free_idx + 1;
        }
        None
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let old_value = self.page_map.get(ppn.0);
        assert!(old_value, "Frame in #{} is not used", ppn.0);
        self.page_map.set(ppn.0, false);
        let mut idx = ppn.0;
        if idx == self.current - 1 {
            while !self.page_map.get(idx) {
                self.current = idx;
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
    }
}

type FrameAllocatorImpl = SimpleAllocator;
lazy_static! {
    pub static ref FRAME_ALLOCATOR: Arc<Mutex<FrameAllocatorImpl>> = {
        let (begin, end) = MEMORY_INFO.get_frame_space_range();
        Arc::new(Mutex::new(FrameAllocatorImpl::new(begin, end)))
    };
}

pub struct FrameTracker {
    pub base_ppn: PhysPageNum,
    pub len: usize,
}

impl FrameTracker {
    pub fn new(base_ppn: PhysPageNum, len: usize) -> Self {
        for idx in 0..len {
            let bytes_array = PhysPageNum(base_ppn.0 + idx).get_bytes_array();
            for i in bytes_array {
                *i = 0;
            }
        }
        Self { base_ppn, len }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        let mut allocator = FRAME_ALLOCATOR.lock();
        for i in 0..self.len {
            allocator.dealloc(PhysPageNum(self.base_ppn.0 + i));
        }
    }
}

pub fn alloc_frame(page_size: usize) -> Option<FrameTracker> {
    FRAME_ALLOCATOR.lock().alloc_contiguous_pages(page_size).map(|ppn| {
        FrameTracker::new(ppn, page_size)
    })
}
