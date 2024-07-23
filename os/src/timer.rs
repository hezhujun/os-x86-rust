use crate::arch::x86::outb;


const INPUT_FREQUENCY: usize = 1193180;
const IRQ0_FREQUENCY: usize = 100;
const COUNTER0_VALUE: usize = INPUT_FREQUENCY / IRQ0_FREQUENCY;
const COUNTER0_PORT: u16 = 0x40;
const PIT_CONTROL_PORT: u16 = 0x43;

pub fn init() {
    outb(PIT_CONTROL_PORT, 0u8 << 6 | 3 << 4 | 2 << 1);
    outb(COUNTER0_PORT, (COUNTER0_VALUE & 0xff) as u8);
    outb(COUNTER0_PORT, ((COUNTER0_VALUE >> 8) & 0xff) as u8);
}
