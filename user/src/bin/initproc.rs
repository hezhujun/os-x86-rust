#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::*;

static mut FORK_COUNT: isize = 0;

#[no_mangle]
fn main() -> isize {
    let mut shell_pid = exec_shell();
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
        if pid == shell_pid {
            shell_pid = exec_shell();
        }
    }
    0
}

fn exec_shell() -> isize {
    let shell_pid = fork();
    if shell_pid < 0 {
        println!("run user_shell error in fork() {}", shell_pid);
        loop {}
    }
    if shell_pid == 0 {
        let ret = exec("user_shell\0", &[core::ptr::null::<u8>()]);
        if ret != 0 {
            println!("run user_shell error in exec() {}", ret);
            loop {}
        }
        0
    } else {
        shell_pid
    }
}
