
mod define;

use core::arch::{asm, global_asm};
use alloc::sync::Arc;
use define::*;
use spin::Mutex;
use IrqType::TIME;
use crate::arch::x86::{outb, DescriptorTablePointer, GateDescriptor};
use crate::arch::x86::pic::{self, OCW2};
use crate::config::HIGH_ADDRESS_BASE;

global_asm!(include_str!("trap.S"));

const IDT_LEN: usize = 0x31;
const IDT_MAX_LEN: usize = 256;

#[no_mangle]
pub extern "C" fn intr_handler(intr: u32, error_code: u32, eip: u32, cs: u32) {
    debug!("intr #{}({:#x}) error code {} {} eip {:#x} cs {:#x}", intr, intr, error_code, IrqErrorCode(error_code), eip, cs);
    assert!((intr >> 8) == 0);
    let intr: u8 = (intr & 0xff).try_into().unwrap();
    match intr {
        IrqType::TIME | IrqType::KEYBOARD | IrqType::IRQ_0X22 | IrqType::IRQ_0X23 | IrqType::IRQ_0X24 | IrqType::IRQ_0X25 | IrqType::IRQ_0X26 | IrqType::IRQ_0X27 => {
            assert_eq!(pic::OCW2::new(false, false, true, 0).0, 0x20);
            outb(PIC_M_CTRL, pic::OCW2::new(false, false, true, 0).0);
        },
        IrqType::IRQ_0X28 | IrqType::IRQ_0X29 | IrqType::IRQ_0X2A | IrqType::IRQ_0X2B | IrqType::IRQ_0X2C | IrqType::IRQ_0X2D | IrqType::IRQ_0X2E | IrqType::IRQ_0X2F => {
            assert_eq!(pic::OCW2::new(false, false, true, 0).0, 0x20);
            outb(PIC_S_CTRL, pic::OCW2::new(false, false, true, 0).0);
            outb(PIC_M_CTRL, pic::OCW2::new(false, false, true, 0).0);
        },
        _ => {
            loop {
                
            }
        }
    }
}

// #[no_mangle]
// pub extern "C" fn intr_handler() {
//     debug!("intr_handler");
//     loop {
        
//     }
// }

/// 主片的控制端口
const PIC_M_CTRL: u16 = 0x20;
/// 主片的数据端口
const PIC_M_DATA: u16 = 0x21;
/// 主片的控制端口
const PIC_S_CTRL: u16 = 0xA0;
/// 主片的数据端口
const PIC_S_DATA: u16 = 0xA1;

fn pic_init() {
    // 初始化主片
    assert_eq!(pic::ICW1::new(true, false, false).0, 0x11u8);
    outb(PIC_M_CTRL, pic::ICW1::new(true, false, false).0);  // ICW1: 边缘触发，级联 8259，需要ICW4
    outb(PIC_M_DATA, pic::ICW2(0x20).0);  // ICW2: 起始中断向量为 0x20
    assert_eq!(pic::ICW3::master(2).0, 0x04u8);
    outb(PIC_M_DATA, pic::ICW3::master(2).0);  // ICW3: IR2 接从片
    assert_eq!(pic::ICW4::uPM.bits(), 0x01u8);
    outb(PIC_M_DATA, pic::ICW4::uPM.bits());  // ICW4: 8086 模式，正常 EOI

    // 初始化从片
    assert_eq!(pic::ICW1::new(true, false, false).0, 0x11u8);
    outb(PIC_S_CTRL, pic::ICW1::new(true, false, false).0);  // ICW1: ICW1: 边缘触发，级联 8259，需要ICW4
    outb(PIC_S_DATA, pic::ICW2(0x28).0);  // ICW2: 起始中断向量为 0x28
    assert_eq!(pic::ICW3::slaver(2).0, 0x02u8);
    outb(PIC_S_DATA, pic::ICW3::slaver(2).0);  // ICW3: 设置从片连接到主片的IR2引脚
    assert_eq!(pic::ICW4::uPM.bits(), 0x01u8);
    outb(PIC_S_DATA, pic::ICW4::uPM.bits());  // ICW4: 8086 模式，正常 EOI

    // 只打开时钟中断
    outb(PIC_M_DATA, 0xfe);
    outb(PIC_S_DATA, 0xff);
}

pub fn init() {
    extern "C" {
        fn intr_table();
    }

    define::init();
    pic_init();

    let idt_pointer = DescriptorTablePointer::new((intr_table as usize + HIGH_ADDRESS_BASE).try_into().unwrap(), (IDT_MAX_LEN * 8 - 1).try_into().unwrap());
    unsafe {
        asm!("lidt [{}]", in(reg) &idt_pointer);
        info!("intr::init done");
        asm!("sti");
    }
}
