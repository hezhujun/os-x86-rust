use super::IntrContext;
use super::IrqType;
use super::INTR_HANDLER_TABLE;
use crate::arch::x86::pic;
use crate::arch::x86::outb;
use crate::schedule::suspend_current_and_run_next;
use crate::drivers::keyboard::handle_keyboard_intr;
use crate::timer::check_timer;
use crate::timer::update_time;

/// 主片的控制端口
const PIC_M_CTRL: u16 = 0x20;
/// 主片的数据端口
const PIC_M_DATA: u16 = 0x21;
/// 主片的控制端口
const PIC_S_CTRL: u16 = 0xA0;
/// 主片的数据端口
const PIC_S_DATA: u16 = 0xA1;

pub fn init() {
    // 初始化主片
    assert_eq!(pic::ICW1::new(true, false, false).0, 0x11u8);
    outb(pic::ICW1::new(true, false, false).0, PIC_M_CTRL);  // ICW1: 边缘触发，级联 8259，需要ICW4
    outb(pic::ICW2(0x20).0, PIC_M_DATA);  // ICW2: 起始中断向量为 0x20
    assert_eq!(pic::ICW3::master(2).0, 0x04u8);
    outb(pic::ICW3::master(2).0, PIC_M_DATA);  // ICW3: IR2 接从片
    assert_eq!(pic::ICW4::uPM.bits(), 0x01u8);
    outb(pic::ICW4::uPM.bits(), PIC_M_DATA);  // ICW4: 8086 模式，正常 EOI

    // 初始化从片
    assert_eq!(pic::ICW1::new(true, false, false).0, 0x11u8);
    outb(pic::ICW1::new(true, false, false).0, PIC_S_CTRL);  // ICW1: ICW1: 边缘触发，级联 8259，需要ICW4
    outb(pic::ICW2(0x28).0, PIC_S_DATA);  // ICW2: 起始中断向量为 0x28
    assert_eq!(pic::ICW3::slaver(2).0, 0x02u8);
    outb(pic::ICW3::slaver(2).0, PIC_S_DATA);  // ICW3: 设置从片连接到主片的IR2引脚
    assert_eq!(pic::ICW4::uPM.bits(), 0x01u8);
    outb(pic::ICW4::uPM.bits(), PIC_S_DATA);  // ICW4: 8086 模式，正常 EOI

    // 只打开时钟中断
    outb(0xfc, PIC_M_DATA);
    outb(0xff, PIC_S_DATA);

    register_pic_intr();
}

fn register_pic_intr() {
    let mut intr_handler_table = INTR_HANDLER_TABLE.lock();
    intr_handler_table[IrqType::TIME as usize] = time_intr_handler;
    intr_handler_table[IrqType::KEYBOARD as usize] = keyboard_intr_handler;
    intr_handler_table[IrqType::IRQ_0X22 as usize] = pic_master_intr_handler;
    intr_handler_table[IrqType::IRQ_0X23 as usize] = pic_master_intr_handler;
    intr_handler_table[IrqType::IRQ_0X24 as usize] = pic_master_intr_handler;
    intr_handler_table[IrqType::IRQ_0X25 as usize] = pic_master_intr_handler;
    intr_handler_table[IrqType::IRQ_0X26 as usize] = pic_master_intr_handler;
    intr_handler_table[IrqType::IRQ_0X27 as usize] = pic_master_intr_handler;

    intr_handler_table[IrqType::IRQ_0X28 as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X29 as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X2A as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X2B as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X2C as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X2D as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X2E as usize] = pic_slaver_intr_handler;
    intr_handler_table[IrqType::IRQ_0X2F as usize] = pic_slaver_intr_handler;
}

fn time_intr_handler(intr_context: &mut IntrContext) {
    update_time();
    check_timer();

    assert_eq!(pic::OCW2::new(false, false, true, 0).0, 0x20);
    outb(pic::OCW2::new(false, false, true, 0).0, PIC_M_CTRL);
    
    suspend_current_and_run_next();
}

fn keyboard_intr_handler(intr_context: &mut IntrContext) {
    handle_keyboard_intr();
    assert_eq!(pic::OCW2::new(false, false, true, 0).0, 0x20);
    outb(pic::OCW2::new(false, false, true, 0).0, PIC_M_CTRL);
}

fn pic_master_intr_handler(intr_context: &mut IntrContext) {
    assert_eq!(pic::OCW2::new(false, false, true, 0).0, 0x20);
    outb(pic::OCW2::new(false, false, true, 0).0, PIC_M_CTRL);
}

fn pic_slaver_intr_handler(intr_context: &mut IntrContext) {
    assert_eq!(pic::OCW2::new(false, false, true, 0).0, 0x20);
    outb(pic::OCW2::new(false, false, true, 0).0, PIC_S_CTRL);
    outb(pic::OCW2::new(false, false, true, 0).0, PIC_M_CTRL);
}
