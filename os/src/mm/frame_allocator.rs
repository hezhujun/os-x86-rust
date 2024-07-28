use core::ops::Drop;
use core::option::Option;
use alloc::sync::Arc;
use spin::Mutex;

use crate::{config::*, mm::{PhysAddr, VirtAddr, VirtPageNum}};
use super::{PhysPageNum, MEMORY_INFO};
use crate::utils::*;

pub trait FrameAllocator {
    fn alloc(&mut self) -> Option<usize>;
    fn alloc_contiguous_pages(&mut self, count: usize) -> Option<usize>;
    fn dealloc(&mut self, idx: usize);
}

pub struct SimpleAllocator<'a, const N: usize> {
    page_map: &'a mut Bitmap<N>,
    base: usize,
    begin: usize,
    end: usize,
    current: usize,
}

impl<'a, const N: usize> SimpleAllocator<'a, { N }> {
    fn new(page_map: &'a mut Bitmap<N>, base: usize, begin: usize, end: usize, current: usize) -> Self {
        assert!(begin < end);
        assert!(begin <= current);
        assert!(current <= end);
        assert!((end - begin) <= N * 8, "end {:#x} begin {:#x} end - begin {:#x} N * 8 {:#x}", end, begin, end - begin, N * 8);
        Self { page_map, base, begin, end, current}
    }

    fn inner_index(&self, outer_index: usize) -> usize {
        outer_index - self.base
    }

    fn set_bitmap(&mut self, idx: usize, value: bool) {
        self.page_map.set(self.inner_index(idx), value)
    }

    fn get_bitmap(&self, idx: usize) -> bool {
        self.page_map.get(self.inner_index(idx))
    }
}

impl<const N: usize> FrameAllocator for SimpleAllocator<'_, { N }> {
    
    fn alloc(&mut self) -> Option<usize> {
        for idx in self.begin..self.current {
            if !self.get_bitmap(idx) {
                self.set_bitmap(idx, true);
                return Some(idx);
            }
        }
        if self.current < self.end {
            self.set_bitmap(self.current, true);
            self.current += 1;
            return Some(self.current - 1);
        }
        None
    }

    fn alloc_contiguous_pages(&mut self, count: usize) -> Option<usize> {
        if self.end - self.current >= count {
            for idx in 0..count {
                self.set_bitmap(self.current + idx, true);
            }
            self.current += count;
            return Some(self.current - count);
        }
        let mut idx = self.begin;
        while idx < self.end {
            let mut free_idx:usize = 0;
            while free_idx < count && !self.get_bitmap(idx + free_idx) {
                free_idx += 1;
            }
            if free_idx == count {
                for i in 0..count {
                    self.set_bitmap(idx + i, true);
                }
                return Some(idx);
            }
            idx += free_idx + 1;
        }
        None
    }

    fn dealloc(&mut self, idx: usize) {
        let old_value = self.get_bitmap(idx);
        assert!(old_value, "Frame in #{} is not used", idx);
        self.set_bitmap(idx, false);
        let mut idx = idx;
        if idx == self.current - 1 {
            while !self.get_bitmap(idx) {
                self.current = idx;
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
    }
}

type PhysFrameAllocatorImpl = SimpleAllocator<'static, PHYS_FRAME_BITMAP_SIZE>;
type KernelVirtFrameAllocatorImpl = SimpleAllocator<'static, KERNEL_VIRT_FRAME_BITMAP_SIZE>;
lazy_static! {
    static ref PHYS_FRAME_ALLOCATOR: Arc<Mutex<PhysFrameAllocatorImpl>> = {
        let begin_ppn = PhysPageNum::from(PhysAddr(FREE_PHYS_FRAME_BEGIN_ADDRESS));
        let end_ppn = MEMORY_INFO.get_frame_space_end();
        let bitmap = unsafe { (PHYS_FRAME_BITMAP_VIRT_ADDRESS as *mut Bitmap<PHYS_FRAME_BITMAP_SIZE>).as_mut().unwrap() };
        Arc::new(Mutex::new(SimpleAllocator::new(bitmap, 0, begin_ppn.0, end_ppn.0, begin_ppn.0)))
    };

    static ref KERNEL_VIRT_FRAME_ALLOCATOR: Arc<Mutex<KernelVirtFrameAllocatorImpl>> = {
        let begin_vpn = VirtPageNum::from(VirtAddr(FREE_KERNEL_VIRT_FRAME_BEGIN_ADDRESS));
        let end_vpn = VirtPageNum::from(VirtAddr(FREE_KERNEL_VIRT_FRAME_END_ADDRESS));
        let bitmap = unsafe { (KERNEL_VIRT_FRAME_BITMAP_VIRT_ADDRESS as *mut Bitmap<KERNEL_VIRT_FRAME_BITMAP_SIZE>).as_mut().unwrap() };
        Arc::new(Mutex::new(SimpleAllocator::new(bitmap, HIGH_ADDRESS_BASE >> 12, begin_vpn.0, end_vpn.0, begin_vpn.0)))
    };
}

pub struct PhysFrameStub {
    pub base_ppn: PhysPageNum,
    pub len: usize,
}

impl PhysFrameStub {
    fn new(base_ppn: PhysPageNum, len: usize) -> Self {
        Self { base_ppn, len }
    }
}

impl Drop for PhysFrameStub {
    fn drop(&mut self) {
        let mut allocator = PHYS_FRAME_ALLOCATOR.lock();
        for i in 0..self.len {
            allocator.dealloc(self.base_ppn.0 + i);
        }
    }
}

pub fn alloc_phys_frame(page_size: usize) -> Option<PhysFrameStub> {
    PHYS_FRAME_ALLOCATOR.lock().alloc_contiguous_pages(page_size).map(|ppn| {
        PhysFrameStub::new(PhysPageNum(ppn), page_size)
    })
}

pub struct VirtFrameStub {
    pub base_vpn: VirtPageNum,
    pub len: usize,
}

impl VirtFrameStub {
    fn new(base_vpn: VirtPageNum, len: usize) -> Self {
        Self { base_vpn, len }
    }
}

impl Drop for VirtFrameStub {
    fn drop(&mut self) {
        let mut allocator = KERNEL_VIRT_FRAME_ALLOCATOR.lock();
        for i in 0..self.len {
            allocator.dealloc(self.base_vpn.0 + i);
        }
    }
}

pub fn alloc_kernel_virt_frame(page_size: usize) -> Option<VirtFrameStub> {
    KERNEL_VIRT_FRAME_ALLOCATOR.lock().alloc_contiguous_pages(page_size).map(|ppn| {
        VirtFrameStub::new(VirtPageNum(ppn), page_size)
    })
}
