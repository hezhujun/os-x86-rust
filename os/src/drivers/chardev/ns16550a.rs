use bitflags::bitflags;

use crate::arch::x86::Register;

use super::CharDevice;

bitflags! {
    pub struct InterruptEnableRegister: u8 {
        const RX_AVAILABLE = 1;
        const TX_EMPTY = 1 << 1;
    }

    pub struct LineStatusRegister: u8 {
        const DATA_AVAILABLE = 1;
        const THR_EMPTY = 1 << 6;
    }

    pub struct ModemControlRegister: u8 {
        const DATA_TERMINAL_READY = 1;
        const REQUEST_TO_SEND = 1 << 1;
        const AUXILIARY_OUTPUT1 = 1 << 2;
        const AUXILIARY_OUTPUT2 = 1 << 3;
    }
}

#[repr(C)]
struct ReadWithoutDLAB {
    rbr: Register,
    ier: Register,
    iir: Register,
    lcr: Register,
    mcr: Register,
    lsr: Register,
    msr: Register,
    scr: Register,
}

impl ReadWithoutDLAB {
    fn new() -> Self {
        let base = Register::new(0x3F8);
        let base1 = Register::new(0x3F9);
        let base2 = Register::new(0x3FA);
        let base3 = Register::new(0x3FB);
        let base4 = Register::new(0x3FC);
        let base5 = Register::new(0x3FD);
        let base6 = Register::new(0x3FE);
        let base7 = Register::new(0x3FF);
        Self { rbr: base, ier: base1, iir: base2, lcr: base3, mcr: base4, lsr: base5, msr: base6, scr: base7 }
    }
}

struct WriteWithoutDLAB {
    thr: Register,
    ier: Register,
    fcr: Register,
    lcr: Register,
    mcr: Register,
    _padding1: Register,
    _padding2: Register,
    scr: Register,
}

impl WriteWithoutDLAB {
    fn new() -> Self {
        let base = Register::new(0x3F8);
        let base1 = Register::new(0x3F9);
        let base2 = Register::new(0x3FA);
        let base3 = Register::new(0x3FB);
        let base4 = Register::new(0x3FC);
        let base5 = Register::new(0x3FD);
        let base6 = Register::new(0x3FE);
        let base7 = Register::new(0x3FF);
        Self { thr: base, ier: base1, fcr: base2, lcr: base3, mcr: base4, _padding1: base5, _padding2: base6, scr: base7 }
    }
}

pub struct NS16550a {
    read_without_dlab: ReadWithoutDLAB,
    write_without_dlab: WriteWithoutDLAB,
}

impl NS16550a {
    pub fn new() -> Self {
        Self { 
            read_without_dlab: ReadWithoutDLAB::new() , 
            write_without_dlab: WriteWithoutDLAB::new(),
        }
    }
}

impl CharDevice for NS16550a {
    fn init(&self) {
        let mcr = ModemControlRegister::DATA_TERMINAL_READY 
            | ModemControlRegister::REQUEST_TO_SEND 
            | ModemControlRegister:: AUXILIARY_OUTPUT2;
        self.read_without_dlab.mcr.write_u8(mcr.bits());
    }

    fn read(&self) -> u8 {
        loop {
            let lsr = LineStatusRegister::from_bits_truncate(self.read_without_dlab.lsr.read_u8());
            if lsr.contains(LineStatusRegister::DATA_AVAILABLE) {
                return self.read_without_dlab.rbr.read_u8()
            }
        }
    }

    fn write(&self, ch: u8) {
        loop {
            let lsr = LineStatusRegister::from_bits_truncate(self.read_without_dlab.lsr.read_u8());
            if lsr.contains(LineStatusRegister::THR_EMPTY) {
                self.write_without_dlab.thr.write_u8(ch);
                return
            }
        }
    }

    fn handle_irq(&self) {
        
    }
}