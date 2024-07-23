#![feature(linkage)]
#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod syscall;
#[macro_use]
pub mod console;

use syscall::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("Cannot find main!");
}

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: isize) -> isize { sys_exit(exit_code) }
