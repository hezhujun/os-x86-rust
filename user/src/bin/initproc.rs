#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::*;

static mut FORK_COUNT: isize = 0;

#[no_mangle]
fn main() -> isize {
    println!("[initproc] pid {}", getpid());
    fork_test();
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
    0
}

fn fork_test() {
    unsafe {
        println!("FORK_COUNT {}", FORK_COUNT);
        FORK_COUNT += 1;
        // if FORK_COUNT >= 100 {
        //     return;
        // }
    }
    let ret = fork();
    if ret == 0 {
        println!("I am child process pid {}", getpid());
        
        exec("hello_world\0", &[]);
        assert!(false);
        loop {}
    }
}
