#![feature(linkage)]
#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate core;

use buddy_system_allocator::LockedHeap;

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

use syscall::*;

const USER_HEAP_SIZE: usize = 0x1000;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    unsafe {
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    exit(main())
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("Cannot find main!");
}


pub fn read(fd: usize, buf: &mut [u8]) -> isize { sys_read(fd, buf) }
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: isize) -> ! { sys_exit(exit_code) }
pub fn yield_() -> isize { sys_yield() }
pub fn get_time() -> isize { sys_get_time() }
pub fn getpid() -> isize { sys_getpid() }
pub fn fork() -> isize { sys_fork() }
pub fn exec(path: &str, args: &[*const u8]) -> isize { sys_exec(path, args) }
pub fn wait(exit_code: &mut isize) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => { yield_(); }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
pub fn waitpid(pid: usize, exit_code: &mut isize) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => { yield_(); }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn sleep(sleep_ms: usize) {
    sys_sleep(sleep_ms);
}

pub fn thread_create(entry: usize, arg: usize) -> isize {
    sys_thread_create(entry, arg)
}
pub fn gettid() -> isize {
    sys_gettid()
}
pub fn waittid(tid: usize) -> isize {
    loop {
        match sys_waittid(tid) {
            -2 => {
                yield_();
            }
            exit_code => return exit_code,
        }
    }
}

pub fn mutex_create() -> isize {
    sys_mutex_create(false)
}

pub fn mutex_blocking_create() -> isize {
    sys_mutex_create(true)
}

pub fn mutex_lock(mutex_id: usize) -> isize {
    sys_mutex_lock(mutex_id)
}

pub fn mutex_unlock(mutext_id: usize) -> isize {
    sys_mutex_unlock(mutext_id)
}

pub fn semaphore_create(res_count: usize) -> isize {
    sys_semaphore_create(res_count)
}

pub fn semaphore_up(sem_id: usize) -> isize {
    sys_semaphore_up(sem_id)
}

pub fn semaphore_down(sem_id: usize) -> isize {
    sys_semaphore_down(sem_id)
}

pub fn condvar_create() -> isize {
    sys_condvar_create()
}

pub fn condvar_signal(condvar_id: usize) -> isize {
    sys_condvar_signal(condvar_id)
}

pub fn condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    sys_condvar_wait(condvar_id, mutex_id)
}
