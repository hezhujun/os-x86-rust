use core::arch::asm;

pub struct ByteReadPort {
    pub port: u16,
}

impl ByteReadPort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn read(&self) -> u8 {
        let mut value: u32;
        unsafe {
            asm!(
                "mov edx, eax",
                "in al, dx",
                inout("eax") self.port as u32 => value,
                out("edx") _,
            );
        }
        (value & 0xff).try_into().unwrap()
    }
}

pub struct ByteWritePort {
    pub port: u16,
}

impl ByteWritePort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn write(&self, value: u8) {
        unsafe {
            asm!(
                "mov edx, {0}",
                "out dx, al",
                in(reg) self.port as u32,
                in("eax") value as u32,
                out("edx") _,
            );
        }
    }
}

pub struct DoubleByteReadPort {
    pub port: u16,
}

impl DoubleByteReadPort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn read(&self) -> u16 {
        let mut value: u32;
        unsafe {
            asm!(
                "mov edx, eax",
                "in ax, dx",
                inout("eax") self.port as u32 => value,
                out("edx") _,
            );
        }
        (value & 0xffff).try_into().unwrap()
    }
}

pub struct DoubleByteWritePort {
    pub port: u16,
}

impl DoubleByteWritePort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn write(&self, value: u16) {
        unsafe {
            asm!(
                "mov edx, {0}",
                "out dx, ax",
                in(reg) self.port as u32,
                in("eax") value as u32,
                out("edx") _,
            );
        }
    }
}

pub struct ByteWriteByteReadPort {
    pub port: u16,
}

impl ByteWriteByteReadPort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn write(&self, value: u8) {
        unsafe {
            asm!(
                "mov edx, {0}",
                "out dx, al",
                in(reg) self.port as u32,
                in("eax") value as u32,
                out("edx") _,
            );
        }
    }

    pub fn read(&self) -> u8 {
        let mut value: u32;
        unsafe {
            asm!(
                "mov edx, eax",
                "in al, dx",
                inout("eax") self.port as u32 => value,
                out("edx") _,
            );
        }
        (value & 0xff).try_into().unwrap()
    }
}


pub struct ByteWriteDoubleByteReadPort {
    pub port: u16,
}

impl ByteWriteDoubleByteReadPort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn write(&self, value: u8) {
        unsafe {
            asm!(
                "mov edx, {0}",
                "out dx, al",
                in(reg) self.port as u32,
                in("eax") value as u32,
                out("edx") _,
            );
        }
    }

    pub fn read(&self) -> u16 {
        let mut value: u32;
        unsafe {
            asm!(
                "mov edx, eax",
                "in ax, dx",
                inout("eax") self.port as u32 => value,
                out("edx") _,
            );
        }
        (value & 0xffff).try_into().unwrap()
    }
}
