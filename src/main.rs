#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(panic_info_message)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod lang_items;
mod arch;
mod drivers;
mod console;
mod boards;
mod mm;
use drivers::chardev::UART;
use drivers::screen::*;
use drivers::CharDevice;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop {}
// }

#[no_mangle]
pub fn main() -> ! {
    UART.init();
    print!("Hello");
    println!(" world");
    print!("1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz");
    print!("1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqrstuvwxyz");
    loop {}
}
