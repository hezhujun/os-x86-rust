mod define;

pub use define::*;

use crate::intr::{IntrContext, INTR_HANDLER_TABLE};


pub fn init() {
    INTR_HANDLER_TABLE.lock()[0x80] = syscall_intr_handler;
}

fn syscall_intr_handler(intr_context: IntrContext) {
    let syscall_id = intr_context.eax;
    let param1 = intr_context.ebx;
    let param2 = intr_context.ecx;
    let param3 = intr_context.edx;

    let ret = match syscall_id {
        SYSCALL_WRITE => sys_write(param1, param2 as *const u8, param3),
        SYSCALL_EXIT=> sys_exit(param1 as isize),
    };
}

fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        1 => {
            let text = unsafe { 
                let v = core::slice::from_raw_parts(buf, len);
                core::str::from_utf8_unchecked(v)
            };
            crate::drivers::Screen.print(text);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

fn sys_exit(exit_code: isize) -> ! {
    debug!("sys_exit");
    loop {
    }
}
