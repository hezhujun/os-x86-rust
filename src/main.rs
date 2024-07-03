#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(panic_info_message)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(alloc_error_handler)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_comparisons)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod console;

extern crate alloc;
mod config;
mod logger;
mod lang_items;
mod arch;
mod drivers;
mod boards;
mod mm;
mod intr;
mod timer;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop {}
// }

#[no_mangle]
pub fn main() -> ! {
    clear_bss();
    let _ = logger::init();
    info!("Hello world!");
    mm::init();
    intr::init();
    loop {}
    timer::init();
    info!("Hello world!");
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
