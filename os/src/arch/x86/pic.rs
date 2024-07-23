
use bitflags::bitflags;
pub struct ICW1(pub u8);

impl ICW1 {
    pub fn new(ic4: bool, sngl: bool, ltim: bool) -> Self {
        let mut v = 0u8;
        if ic4 {
            v |= 1;
        }
        if sngl {
            v |= 1 << 1;
        }
        if ltim {
            v |= 1 << 3;
        }
        v |= 1 << 4;
        Self(v)
    }
}

pub struct ICW2(pub u8);
pub struct ICW3(pub u8);

impl ICW3 {
    pub fn master(idx: u8) -> Self {
        Self(1u8 << idx)
    }

    pub fn slaver(idx: u8) -> Self {
        Self(idx)
    }
}


bitflags! {
    pub struct ICW4: u8 {
        const uPM = 1;
        const AEOI = 1 << 1;
        const M_S = 1 << 2;
        const BUF = 1 << 3;
        const SFNM = 1 << 4;
    }

    pub struct OCW1: u8 {
        const IRQ0 = 1;
        const IRQ1 = 1 << 1;
        const IRQ2 = 1 << 2;
        const IRQ3 = 1 << 3;
        const IRQ4 = 1 << 4;
        const IRQ5 = 1 << 5;
        const IRQ6 = 1 << 6;
        const IRQ7 = 1 << 7;
    }
}

pub struct OCW2(pub u8);
impl OCW2 {
    pub fn new(R: bool, SL: bool, EOI: bool, idx: u8) -> Self {
        let mut v = 0u8;
        if R {
            v |= 1 << 7;
        }
        if SL {
            v |= 1 << 6;
        }
        if EOI {
            v |= 1 << 5;
        }
        v |= idx & 0b111;
        Self(v)
    }
}
