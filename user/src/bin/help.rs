#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static APPS: &[&str] = &[
    "hello_world",
    "hello_world_a",
    "forktest_simple",
    "forktest",
    "forktest2",
    "forktree",
    "threads",
    "threads_arg",
    "adder_mutex_spin",
    "adder_mutex_blocking",
    "mpsc_sem",
    "condsync_sem",
    "condsync_condvar",
    "barrier_condvar",
];

#[no_mangle]
fn main() -> isize {
    println!("executable program list:");
    for app_name in APPS {
        println!("{}", app_name);
    }
    0
}
