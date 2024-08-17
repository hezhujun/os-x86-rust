use core::ops::Range;
use core::iter::*;
use core::option::*;
use core::convert::From;
use crate::config::MEMORY_PAGE_SIZE;

#[derive(Clone, Copy)]
pub struct PhysAddr(pub usize);
pub type PA = PhysAddr;
#[derive(Clone, Copy)]
pub struct VirtAddr(pub usize);
pub type VA = VirtAddr;

impl PhysAddr {
    pub fn phys_page_num_floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 >> 12)
    }

    pub fn phys_page_num_ceil(&self) -> PhysPageNum {
        let ret = self.phys_page_num_floor();
        if self.0 & 0xfff != 0 {
            PhysPageNum(ret.0 + 1)
        } else {
            ret
        }
    }
}

impl VirtAddr {
    pub fn virt_page_num_floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 >> 12)
    }

    pub fn virt_page_num_ceil(&self) -> VirtPageNum {
        let ret = self.virt_page_num_floor();
        if self.0 & 0xfff != 0 {
            VirtPageNum(ret.0 + 1)
        } else {
            ret
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysPageNum(pub usize);
pub type PPN = PhysPageNum;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtPageNum(pub usize);
pub type VPN = VirtPageNum;

impl From<PhysAddr> for PhysPageNum {
    fn from(value: PhysAddr) -> Self {
        Self(value.0 >> 12)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(value: VirtAddr) -> Self {
        Self(value.0 >> 12)
    }
}

impl PhysPageNum {
    pub fn base_address(&self) -> PhysAddr {
        PhysAddr(self.0 << 12)
    }
}

impl VirtPageNum {
    pub fn base_address(&self) -> VirtAddr {
        VirtAddr(self.0 << 12)
    }
}

impl VirtPageNum {
    pub fn offset(&self, offset: usize) -> VirtAddr {
        VirtAddr(self.base_address().0 + offset)
    }

    pub fn as_ref<T: Sized>(&self) -> &T {
        assert!(core::mem::size_of::<T>() < MEMORY_PAGE_SIZE);
        unsafe {
            (self.base_address().0 as *const T).as_ref().unwrap()
        }
    }

    pub fn as_mut<T: Sized>(&self) -> &mut T {
        assert!(core::mem::size_of::<T>() < MEMORY_PAGE_SIZE);
        unsafe {
            (self.base_address().0 as *mut T).as_mut().unwrap()
        }
    }

    pub fn as_bytes_array_ref(&self) -> &[u8; MEMORY_PAGE_SIZE] {
        unsafe {
            (self.base_address().0 as *const [u8; MEMORY_PAGE_SIZE]).as_ref().unwrap()
        }
    }

    pub fn as_bytes_array_mut<T: Sized>(&self) -> &mut [u8; MEMORY_PAGE_SIZE] {
        assert!(core::mem::size_of::<T>() < MEMORY_PAGE_SIZE);
        unsafe {
            (self.base_address().0 as *mut [u8; MEMORY_PAGE_SIZE]).as_mut().unwrap()
        }
    }

    pub fn get_ref<T: Sized, F: for<'a> FnOnce(&'a T) -> ()>(&self, offset: usize, f: F) {
        if offset + core::mem::size_of::<T>() > MEMORY_PAGE_SIZE {
            return;
        }
        let address = self.base_address().0 + offset;
        let value = unsafe { (address as *const T).as_ref() }.unwrap();
        f(value);
    }

    pub fn get_mut<T: Sized, F: for<'a> FnOnce(&'a mut T) -> ()>(&self, offset: usize, f: F) {
        if offset + core::mem::size_of::<T>() > MEMORY_PAGE_SIZE {
            return;
        }
        let address = self.base_address().0 + offset;
        let value = unsafe { (address as *mut T).as_mut() }.unwrap();
        f(value);
    }

    pub fn get_bytes_array_ref<F: for<'a> FnOnce(&'a [u8; MEMORY_PAGE_SIZE]) -> ()>(&self, f: F) {
        self.get_ref(0, f);
    }

    pub fn get_bytes_array_mut<F: for<'a> FnOnce(&'a mut [u8; MEMORY_PAGE_SIZE]) -> ()>(&self, f: F) {
        self.get_mut(0, f);
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.base_address().0 as *mut u8, MEMORY_PAGE_SIZE)
        }
    }

    pub fn gte_bytes_mut(&self) -> &'static mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.base_address().0 as *mut u8, MEMORY_PAGE_SIZE)
        }
    }
}

impl VirtAddr {
    pub fn as_ref<T>(&self) -> &T {
        unsafe {
            (self.0 as *const T).as_ref().unwrap()
        }
    }

    pub fn as_mut<T>(&self) -> &mut T {
        unsafe {
            (self.0 as *mut T).as_mut().unwrap()
        }
    }
}

pub type VirtPageNumRange = Range<VirtPageNum>;
pub type VPNRange = VirtPageNumRange;

impl From<VirtPageNum> for VirtPageNumRange {
    fn from(value: VirtPageNum) -> Self {
        value..VirtPageNum(value.0 + 1)
    }
}

pub type PhysPageNumRange = Range<PhysPageNum>;
pub type PPNRange = PhysPageNumRange;

impl From<PhysPageNum> for PhysPageNumRange {
    fn from(value: PhysPageNum) -> Self {
        value..PhysPageNum(value.0 + 1)
    }
}

impl Step for VirtPageNum {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end.0 > start.0 {
            Some(end.0 - start.0)
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(VirtPageNum(start.0 + count))
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if start.0 >= count {
            Some(VirtPageNum(start.0 - count))
        } else {
            None
        }
    }
}
