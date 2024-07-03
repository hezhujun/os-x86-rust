
mod model;
mod segment_descriptor;
mod screen;
mod register;
mod pte;
mod gate_descriptor;
pub mod pic;

use core::arch::asm;
pub use model::*;
pub use segment_descriptor::*;
pub use screen::*;
pub use register::*;
pub use pte::*;
pub use gate_descriptor::*;


pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "mov edx, {0}",
            "out dx, al",
            in(reg) port as u32,
            in("eax") value as u32,
            out("edx") _,
        );
    }
}

pub fn outw(port: u16, value: u16) {
    unsafe {
        asm!(
            "mov edx, {0}",
            "out dx, ax",
            in(reg) port as u32,
            in("eax") value as u32,
            out("edx") _,
        );
    }
}
