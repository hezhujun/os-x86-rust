#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> isize {
    for i in 0..50 {
        println!("hello world b {}", i);
    }
    0
}
