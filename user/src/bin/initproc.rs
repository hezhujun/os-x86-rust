#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::*;

#[no_mangle]
fn main() -> isize {
    loop {
        let mut exit_code: isize = 0;
        let pid = wait(&mut exit_code);
        if pid == -1 {
            yield_();
            continue;
        }
        println!(
            "[initproc] Released a zombie process, pid={}, exit_code={}",
            pid,
            exit_code,
        );
    }
}
