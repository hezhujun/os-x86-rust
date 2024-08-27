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
    exit(main())
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("Cannot find main!");
}

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: isize) -> ! { sys_exit(exit_code) }
pub fn yield_() -> isize { sys_yield() }
pub fn getpid() -> isize { sys_getpid() }
pub fn fork() -> isize { sys_fork() }
pub fn exec(path: &str, args: &[*const u8]) -> isize { sys_exec(path, args) }
pub fn waitpid(pid: isize, exit_code: &mut isize) -> isize { sys_waitpid(pid, exit_code) }
pub fn wait(exit_code: &mut isize) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => { yield_(); }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
