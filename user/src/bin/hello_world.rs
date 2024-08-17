#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> isize {
    for i in 0..10 {
        println!("hello world {}", i);
    }
    0
}
