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
}
