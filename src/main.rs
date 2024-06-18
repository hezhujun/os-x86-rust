#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(panic_info_message)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod lang_items;
mod arch;
mod drivers;
#[macro_use]
mod console;
mod boards;
mod mm;
use drivers::chardev::UART;
use drivers::screen::*;
use drivers::CharDevice;
use mm::*;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop {}
// }

#[no_mangle]
pub fn main() -> ! {
    let esp = arch::x86::ESP.read();
    UART.init();
    println!("Hello world!");
    print!("esp: {:#x}", esp);
    memory_info();
    loop {}
}
