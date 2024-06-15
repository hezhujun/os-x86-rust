#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod driver;
use driver::print_test;
use x86::*;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop {}
// }

#[no_mangle]
pub fn main() -> ! {
    print_test();
    loop {}
}
