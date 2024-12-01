
mod model;
mod segment_descriptor;
mod screen;
mod register;
mod pte;
mod gate_descriptor;
pub mod pic;
mod port;

use core::arch::asm;
pub use model::*;
pub use segment_descriptor::*;
pub use screen::*;
pub use register::*;
pub use pte::*;
pub use gate_descriptor::*;
pub use port::*;


pub fn outb(value: u8, port: u16) {
    unsafe {
        asm!(
            "out dx, al",
            in("edx") port as u32,
            in("eax") value as u32
        );
    }
}

pub fn outw(value: u16, port: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("edx") port as u32,
            in("eax") value as u32
        );
    }
}

pub fn inb(port: u16) -> u8 {
    let mut value: u32;
    unsafe {
        asm!(
            "in al, dx",
            in("edx") port as u32,
            out("eax") value
        );
    }
    (value & 0xff).try_into().unwrap()
}


pub fn inw(port: u16) -> u16 {
    let mut value: u32;
    unsafe {
        asm!(
            "in ax, dx",
            in("edx") port as u32,
            out("eax") value
        );
    }
    (value & 0xffff).try_into().unwrap()
}
