#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::*;

static NUM: usize = 30;

#[no_mangle]
pub fn main() -> isize {
    for _ in 0..NUM {
        let pid = fork();
        if pid == 0 {
            let current_time = get_time();
            let sleep_length =
                (current_time as isize) * (current_time as isize) % 1000 + 1000;
            println!("pid {} sleep for {} ms", getpid(), sleep_length);
            sleep(sleep_length as usize);
            println!("pid {} OK!", getpid());
            exit(0);
        }
    }

    let mut exit_code: isize = 0;
    for _ in 0..NUM {
        assert!(wait(&mut exit_code) > 0);
        assert_eq!(exit_code, 0);
    }
    assert!(wait(&mut exit_code) < 0);
    println!("forktest2 test passed!");
    0
}
