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
