#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(panic_info_message)]
#![allow(incomplete_features)]
#![feature(alloc_error_handler)]
#![feature(step_trait)]

#![feature(generic_const_exprs)]
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
extern crate core;
mod config;
mod logger;
mod lang_items;
mod arch;
mod drivers;
mod boards;
mod mm;
mod intr;
mod timer;
mod process;
mod schedule;
mod syscall;
mod utils;
mod programs;
mod fs;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

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
    drivers::init();
    intr::init();
    syscall::init();
    schedule::init();
    timer::init();
    // schedule::test();
    intr::begin_intr();
    schedule::run_tasks();
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
