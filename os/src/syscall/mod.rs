mod define;
mod process;
mod thread;
mod io;

use define::*;
use process::*;
use thread::*;
use io::*;

use crate::{intr::{set_ldt_entry, IntrContext, INTR_HANDLER_TABLE}, schedule::current_task};
use crate::schedule::check_current_process_status;


pub fn init() {
    INTR_HANDLER_TABLE.lock()[0x80] = syscall_intr_handler;
    set_ldt_entry(0x80, 0b11);
}

fn syscall_intr_handler(intr_context: &mut IntrContext) {
    check_current_process_status();

    let syscall_id = intr_context.eax;
    let param1 = intr_context.ebx;
    let param2 = intr_context.ecx;
    let param3 = intr_context.edx;

    if let Some(task) = current_task() {
        let mut task_inner = task.inner.lock();
        task_inner.intr_cx = *intr_context;
    }

    // debug!("syscall_intr_handler {}", syscall_id);

    let ret = match syscall_id {
        SYSCALL_READ => sys_read(param1, param2 as *mut u8, param3),
        SYSCALL_WRITE => sys_write(param1, param2 as *const u8, param3),
        SYSCALL_EXIT => sys_exit((param1 as isize).try_into().unwrap()),
        SYSCALL_SLEEP => sys_sleep(),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(param1 as *const u8, param2 as *const usize, intr_context),
        SYSCALL_WAITPID => sys_waitpid(param1 as isize, param2 as *mut isize),
        SYSCALL_THREAD_CREATE => sys_thread_create(param1, param2),
        SYSCALL_GETTID => sys_gettid(),
        SYSCALL_WAITTID => sys_waittid(param1) as isize,
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    };

    intr_context.eax = ret as usize;
}
