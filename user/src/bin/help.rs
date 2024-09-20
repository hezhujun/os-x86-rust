#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static APPS: &[&str] = &[
    "hello_world",
    "hello_world_a",
    "hello_world_b",
];

#[no_mangle]
fn main() -> isize {
    println!("executable program list:");
    for app_name in APPS {
        println!("{}", app_name);
    }
    0
}
