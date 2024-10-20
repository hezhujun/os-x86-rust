#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec::Vec;
use user_lib::*;

#[repr(C)]
struct Argument {
    pub ch: char,
    pub rc: isize,
}

extern "C" fn thread_print(arg: *const Argument) {
    println!("thread_print arg address {:#x}", arg as usize);
    let arg = unsafe { &*arg };
    for _ in 0..1000 {
        print!("{}", arg.ch);
    }
    exit(arg.rc)
}

#[no_mangle]
pub fn main() -> i32 {
    let mut v = Vec::new();
    let args = [
        Argument { ch: 'a', rc: 1 },
        Argument { ch: 'b', rc: 2 },
        Argument { ch: 'c', rc: 3 },
    ];
    println!("begin create threads");
    for arg in args.iter() {
        println!("arg address {:#x}", arg as *const _ as usize);
        v.push(thread_create(thread_print as usize, arg as *const _ as usize));
    }
    for tid in v.iter() {
        let exit_code = waittid(*tid as usize);
        println!("thread#{} exited with code {}", tid, exit_code);
    }
    println!("main thread exited.");
    0
}
