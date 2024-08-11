use core::fmt::Display;

#[derive(Clone, Copy)]
pub struct GateDescriptor([u32;2]);

pub const INTR_GATE_ATTR: u8 = 0b1110;

impl GateDescriptor {
    pub fn empty() -> Self {
        Self([0; 2])
    }

    pub fn new(selector: u16, address: u32, is_present: bool, DPL: u8, attr: u8) -> Self {
        let mut low = 0u32;
        let mut high = 0u32;
        low |= (selector as u32) << 16;
        low |= address & 0xffff;
        high |= address & 0xffff0000;
        if is_present {
            high |= 1 << 15;
        }
        high |= (DPL as u32 & 0b11) << 13;
        high |= (attr as u32 & 0b1111) << 8;
        Self([low, high])
    }

    pub fn get_selector(&self) -> u16 {
        ((self.0[0] >> 16) & 0xffff).try_into().unwrap()
    }

    pub fn set_selector(&mut self, selector: u16) {
        self.0[0] &= 0xffff;
        self.0[0] |= (selector as u32) << 16;
    }

    pub fn get_address(&self) -> u32 {
        let mut ret = 0u32;
        ret |= self.0[0] & 0xffff;
        ret |= self.0[1] & 0xffff0000;
        ret
    }

    pub fn set_address(&mut self, address: u32) {
        self.0[0] &= 0xffff0000;
        self.0[0] |= address & 0xffff;
        self.0[1] &= 0xffff;
        self.0[1] |= address & 0xffff0000;
    }

    pub fn is_present(&self) -> bool {
        self.0[1] & (1 << 15) != 0
    }

    pub fn set_DPL(&mut self, dpl: u8) {
        self.0[1] &= !(0b11 << 13);
        self.0[1] |= (dpl as u32 & 0b11) << 13;
    }

    pub fn get_DPL(&self) -> u8 {
        ((self.0[1] >> 13) & 0b11).try_into().unwrap()
    }

    pub fn get_attr(&self) -> u8 {
        ((self.0[1] >> 8) & 0b1111).try_into().unwrap()
    }
}
impl Display for GateDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GateDescriptor[selector: {}, address: {:#x}, is_present: {}, dpl: {}, type: {:#b}]", self.get_selector(), self.get_address(), self.is_present(), self.get_DPL(), self.get_attr())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_descriptor() {
        let selector: u16 = 0xdc;
        let address: u32 = 0xfff888;
        let dpl: u8 = 0b11;
        let attr: u8 = 0b1110;
        let descriptor = GateDescriptor::new(selector, address, true, dpl, attr);
        assert_eq!(selector, descriptor.get_selector());
        assert_eq!(address, descriptor.get_address());
        assert_eq!(true, descriptor.is_present());
        assert_eq!(dpl, descriptor.get_DPL());
        assert_eq!(attr, descriptor.get_attr());
    }

}
