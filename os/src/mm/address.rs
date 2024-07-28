use core::ops::Range;
use core::iter::*;
use core::option::*;
use core::convert::From;
use crate::config::MEMORY_PAGE_SIZE;

#[derive(Clone, Copy)]
pub struct PhysAddr(pub usize);
#[derive(Clone, Copy)]
pub struct VirtAddr(pub usize);

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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtPageNum(pub usize);

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


impl PhysPageNum {
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.base_address().0 as *mut u8, MEMORY_PAGE_SIZE)
        }
    }
}

impl VirtPageNum {
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
    pub fn as_mut_ref<T>(&self) -> &mut T {
        unsafe {
            (self.0 as *mut T).as_mut().unwrap()
        }
    }
}

pub type VPNRange = Range<VirtPageNum>;

impl From<VirtPageNum> for VPNRange {
    fn from(value: VirtPageNum) -> Self {
        value..VirtPageNum(value.0 + 1)
    }
}
