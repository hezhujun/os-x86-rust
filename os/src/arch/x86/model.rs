use core::arch::asm;

use bitflags::bitflags;


bitflags! {
    pub struct Eflags: u32 {
        const CF = 1;
        const PF = 1 << 2;
        const AF = 1 << 4;
        const ZF = 1 << 6;
        const SF = 1 << 7;
        const TF = 1 << 8;
        const IF = 1 << 9;
        const DF = 1 << 10;
        const OF = 1 << 11;
        const IOPL_0 = 1 << 12;
        const IOPL_1 = 1 << 13;
        const NT = 1 << 14;
        const RF = 1 << 16;
        const VM = 1 << 17;
        const AC = 1 << 18;
        const VIF = 1 << 19;
        const VIP = 1 << 20;
        const ID = 1 << 21;
    }

    pub struct Cr0: u32 {
        const PE = 1;
        const MP = 1 << 1;
        const EM = 1 << 2;
        const TS = 1 << 3;
        const ET = 1 << 4;
        const NE = 1 << 5;
        const WP = 1 << 16;
        const AM = 1 << 18;
        const NW = 1 << 29;
        const CD = 1 << 30;
        const PG = 1 << 31;
    }
}

impl Eflags {
    pub fn IOPL(&self) -> u32 {
        (self.bits() >> 12) & 0b11
    }

    pub fn read() -> Self {
        let mut value: u32 = 0;
        unsafe {
            asm!("pushfd", "pop eax", out("eax") value);
        }
        Eflags::from_bits_truncate(value)
    }
}

impl Cr0 {
    pub fn read() -> Self {
        let mut value: u32 = 0;
        unsafe {
            asm!("mov eax, cr0", out("eax") value);
        }
        Cr0::from_bits_truncate(value)
    }

    pub fn write(&self) {
        let value = self.bits();
        unsafe {
            asm!(
                "mov cr0, eax",
                in("eax") value
            );
        }
    }
}

pub struct ESP;

impl ESP {
    pub fn read(&self) -> u32 {
        let mut value: u32 = 0;
        unsafe {
            asm!("mov eax, esp", out("eax") value);
        }
        value
    }

    pub fn write(&self, value: u32) {
        unsafe {
            asm!(
                "mov esp, eax",
                in("eax") value
            );
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AddressRangeDescriptorStructure {
    pub addr_low: u32,
    pub addr_high: u32,
    pub length_low: u32,
    pub length_high: u32,
    pub memory_type: u32,
}

impl AddressRangeDescriptorStructure {
    pub fn empty() -> Self {
        Self { addr_low: 0, addr_high: 0, length_low: 0, length_high: 0, memory_type: 0 }
    }
}

impl AddressRangeDescriptorStructure {
    pub fn get_addr(&self) -> u64 {
        ((self.addr_high as u64) << 32) | self.addr_low as u64
    }

    pub fn get_length(&self) -> u64 {
        ((self.length_high as u64) << 32) | self.length_low as u64
    }

    pub fn is_usable(&self) -> bool {
        self.memory_type == 1
    }
}

pub struct DescriptorTablePointer(u64);

impl DescriptorTablePointer {
    pub fn new(address: u32, limit: u16) -> Self {
        let mut v = limit as u64;
        v |= (address as u64) << 16;
        Self(v)
    }

    pub fn get_limit(&self) -> u16 {
        (self.0 & 0xffff).try_into().unwrap()
    }

    pub fn get_address(&self) -> u32 {
        ((self.0 >> 16) & 0xffff).try_into().unwrap()
    }
}
