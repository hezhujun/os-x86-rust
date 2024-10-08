
mod define;
mod context;
mod pic;

pub use context::*;
pub use define::IrqErrorCode;
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

type IntrHandlerFn = fn(&mut IntrContext);

lazy_static! {
    pub static ref INTR_HANDLER_TABLE: Arc<Mutex<[IntrHandlerFn; IDT_MAX_LEN]>> = Arc::new(Mutex::new([default_intr_handler; IDT_MAX_LEN]));
}

pub fn set_ldt_entry(idx: usize, dpl: usize) {
    define::set_ldt_entry(idx, dpl);
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
pub extern "C" fn intr_handler(mut intr_context: IntrContext) {
    assert_eq!(intr_context.magic, 0x1234);
    let intr = intr_context.intr;
    let error_code = intr_context.error_code;
    let eip = intr_context.eip;
    let cs = intr_context.cs;
    assert!((intr >> 8) == 0);
    let handler = INTR_HANDLER_TABLE.lock()[intr];
    handler(&mut intr_context);

    let eip = intr_context.eip;
    let cs = intr_context.cs;
    let ss = intr_context.ss;
    let esp = intr_context.esp;
    assert_ne!(eip, 0, "intr return #{}({:#x}) error code {} {} eip {:#x} cs {:#x} esp {:#x} ss {:#x}", intr, intr, error_code, IrqErrorCode(error_code), eip, cs, esp, ss);
}

extern "C" {
    pub fn intr_exit();
}

fn default_intr_handler(intr_context: &mut IntrContext) {
    let intr = intr_context.intr;
    let error_code = intr_context.error_code;
    let eip = intr_context.eip;
    let cs = intr_context.cs;
    let ss = intr_context.ss;
    let esp = intr_context.esp;
    debug!("intr #{}({:#x}) error code {} {} eip {:#x} cs {:#x} esp {:#x} ss {:#x}", intr, intr, error_code, IrqErrorCode(error_code), eip, cs, esp, ss);
    info!("no handle intr");
    loop {}
}
