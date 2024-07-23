
mod define;
mod context;
mod pic;

pub use context::*;
use core::arch::{asm, global_asm};
use alloc::sync::Arc;
use define::*;
use spin::Mutex;
use IrqType::TIME;
use crate::arch::x86::{DescriptorTablePointer, GateDescriptor};
use crate::schedule::suspend_current_and_run_next;

global_asm!(include_str!("trap.S"));

const IDT_LEN: usize = 0x31;
const IDT_MAX_LEN: usize = 256;

type IntrHandlerFn = fn(IntrContext);

lazy_static! {
    pub static ref INTR_HANDLER_TABLE: Arc<Mutex<[IntrHandlerFn; IDT_MAX_LEN]>> = Arc::new(Mutex::new([default_intr_handler; IDT_MAX_LEN]));
}

pub fn init() {
    extern "C" {
        fn intr_table();
    }

    define::init();
    pic::init();

    let idt_pointer = DescriptorTablePointer::new((intr_table as usize).try_into().unwrap(), (IDT_MAX_LEN * 8 - 1).try_into().unwrap());
    unsafe {
        asm!("lidt [{}]", in(reg) &idt_pointer);
    }
    info!("intr::init done");
}

pub fn begin_intr() {
    unsafe {
        asm!("sti");
    }
}


#[no_mangle]
pub extern "C" fn intr_handler(intr_context: IntrContext) {
    assert_eq!(intr_context.magic, 0x1234);
    let intr = intr_context.intr;
    let error_code = intr_context.error_code;
    let eip = intr_context.eip;
    let cs = intr_context.cs;
    // debug!("intr #{}({:#x}) error code {} {} eip {:#x} cs {:#x}", intr, intr, error_code, IrqErrorCode(error_code), eip, cs);
    assert!((intr >> 8) == 0);
    INTR_HANDLER_TABLE.lock()[intr](intr_context);
}

extern "C" {
    pub fn intr_exit();
}

fn default_intr_handler(intr_context: IntrContext) {
    let intr = intr_context.intr;
    let error_code = intr_context.error_code;
    let eip = intr_context.eip;
    let cs = intr_context.cs;
    debug!("intr #{}({:#x}) error code {} {} eip {:#x} cs {:#x}", intr, intr, error_code, IrqErrorCode(error_code), eip, cs);
    info!("no handle intr");
    loop {}
}
