use core::arch::asm;
use core::convert::TryInto;

pub struct Register(usize);

impl Register {
    pub fn new(address: usize) -> Self {
        Self(address)
    }
}

impl Register {
    pub fn read_u8(&self) -> u8 {
        let mut value: u32;
        unsafe {
            asm!(
                "mov edx, eax",
                "in al, dx",
                inout("eax") self.0 => value,
                out("edx") _,
            );
        }
        (value & 0xff).try_into().unwrap()
    }

    pub fn read_u16(&self) -> u16 {
        let mut value: u32;
        unsafe {
            asm!(
                "mov edx, eax",
                "in ax, dx",
                inout("eax") self.0 => value,
                out("edx") _,
            );
        }
        (value & 0xffff).try_into().unwrap()
    }

    pub fn write_u8(&self, value: u8) {
        unsafe {
            asm!(
                "mov edx, {0}",
                "out dx, al",
                in(reg) self.0,
                in("eax") value as u32,
                out("edx") _,
            );
        }
    }

    pub fn write_u16(&self, value: u16) {
        unsafe {
            asm!(
                "mov edx, {0}",
                "out dx, ax",
                in(reg) self.0,
                in("eax") value as u32,
                out("edx") _,
            );
        }
    }
}
