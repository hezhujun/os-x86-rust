#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> isize {
    loop {
        for i in 0..2000 {
            println!("hello world a {}", i);
        }
    }
    0
}
