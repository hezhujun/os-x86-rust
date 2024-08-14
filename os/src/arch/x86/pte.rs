use core::{arch::asm, fmt::Display};
use bitflags::bitflags;


bitflags! {
    pub struct PdeFlags: u32 {
        /// Present
        const P = 1;
        /// 1: read/write 0: read
        const RW = 1 << 1;
        /// 1: User level 0: Supervisor level
        const US = 1 << 2;
        /// Page-level Write-Through
        const PWT = 1 << 3;
        /// Page-level Cache Disable
        const PCD = 1 << 4;
        /// Accessed
        const A = 1 << 5;
        /// Dirty
        const D = 1 << 6;
        /// Global
        const G = 1 << 8;
        /// Available
        const AVL = 1 << 9;
    }

    pub struct PteFlags: u32 {
        /// Present
        const P = 1;
        /// 1: read/write 0: read
        const RW = 1 << 1;
        /// 1: User level 0: Supervisor level
        const US = 1 << 2;
        /// Page-level Write-Through
        const PWT = 1 << 3;
        /// Page-level Cache Disable
        const PCD = 1 << 4;
        /// Accessed
        const A = 1 << 5;
        /// Dirty
        const D = 1 << 6;
        /// Page Attribute Table
        const PAT = 1 << 7;
        /// Global
        const G = 1 << 8;
        /// Available
        const AVL = 1 << 9;
    }

    pub struct PdbrFlag: u32 {
        const PWT = 1 << 3;
        const PCD = 1 << 4;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PageDirectoryEntry(pub u32);
impl PageDirectoryEntry {
    pub fn new(address: u32, flag: PdeFlags) -> Self {
        assert!(address & 0xfff == 0);
        Self(address | flag.bits())
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn address(&self) -> u32 {
        self.0 & 0xfffff000
    }

    pub fn set_address(&mut self, address: u32) {
        *self = Self::new(address, self.flag());
    }

    pub fn flag(&self) -> PdeFlags {
        PdeFlags::from_bits_truncate(self.0 & 0xfff)
    }
}


impl Display for PageDirectoryEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PageTableEntry[address: {:#x}, flag: {:#b}]", self.address(), (self.flag().bits() & 0x3ff))
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PageTableEntry(pub u32);
impl PageTableEntry {
    pub fn new(address: u32, flag: PteFlags) -> Self {
        assert!(address & 0xfff == 0);
        Self(address | flag.bits())
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn address(&self) -> u32 {
        self.0 & 0xfffff000
    }

    pub fn flag(&self) -> PteFlags {
        PteFlags::from_bits_truncate(self.0 & 0xfff)
    }

    pub fn set_flag(&mut self, flag: PteFlags) {
        *self = Self::new(self.address(), flag);
    }
}

impl Display for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PageTableEntry[address: {:#x}, flag: {:#b}]", self.address(), (self.flag().bits() & 0x3ff))
    }
}

pub struct PageDirectoryBaseRegister(pub u32);
pub type Cr3 = PageDirectoryBaseRegister;

impl PageDirectoryBaseRegister {
    pub fn new(address: u32, flag: PdbrFlag) -> Self {
        assert!(address & 0xfff == 0);
        Self(address | flag.bits())
    }

    pub fn read() -> Self {
        let mut value: u32 = 0;
        unsafe {
            asm!("mov eax, cr3", out("eax") value);
        }
        Self(value)
    }

    pub fn write(&self) {
        let value = self.0;
        unsafe {
            asm!(
                "mov cr3, eax",
                in("eax") value
            );
        }
    }
}
