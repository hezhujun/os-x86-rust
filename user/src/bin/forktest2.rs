#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, getpid};

#[no_mangle]
pub fn main() -> isize {
    println!("parent start, pid = {}!", getpid());
    let pid = fork();
    if pid == 0 {
        // child process
        println!("hello child process!");
        for i in 0..=10000 {
            if i % 1000 == 0 {
                println!("child process iter {}", i);
            }
        }
        println!("child process exit!");
        0
    } else {
        // parent process
        0
    }
}
