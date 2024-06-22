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
}

impl Eflags {
    pub fn IOPL(&self) -> u32 {
        (self.bits() >> 12) & 0b11
    }
}

pub struct  ESP;

impl ESP {
    pub fn read(&self) -> u32 {
        let mut value: u32 = 0;
        unsafe {
            asm!("mov eax, esp", out("eax") value);
        }
        value
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
