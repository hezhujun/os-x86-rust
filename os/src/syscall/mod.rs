mod define;
mod process;
mod io;

use define::*;
use process::*;
use io::*;

use crate::intr::{set_ldt_entry, IntrContext, INTR_HANDLER_TABLE};


pub fn init() {
    INTR_HANDLER_TABLE.lock()[0x80] = syscall_intr_handler;
    set_ldt_entry(0x80, 0b11);
}

fn syscall_intr_handler(intr_context: IntrContext) {
    let syscall_id = intr_context.eax;
    let param1 = intr_context.ebx;
    let param2 = intr_context.ecx;
    let param3 = intr_context.edx;

    let ret = match syscall_id {
        SYSCALL_WRITE => sys_write(param1, param2 as *const u8, param3),
        SYSCALL_EXIT=> sys_exit((param1 as isize).try_into().unwrap()),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    };
}
