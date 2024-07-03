use crate::{arch::x86::{GateDescriptor, INTR_GATE_ATTR}, config::{CODE_SELECTOR, HIGH_ADDRESS_BASE}, intr::{IDT_LEN, IDT_MAX_LEN}};

extern "C" {
    pub fn intr_entry_0x00();
    pub fn intr_entry_0x01();
    pub fn intr_entry_0x02();
    pub fn intr_entry_0x03();
    pub fn intr_entry_0x04();
    pub fn intr_entry_0x05();
    pub fn intr_entry_0x06();
    pub fn intr_entry_0x07();
    pub fn intr_entry_0x08();
    pub fn intr_entry_0x09();
    pub fn intr_entry_0x0a();
    pub fn intr_entry_0x0b();
    pub fn intr_entry_0x0c();
    pub fn intr_entry_0x0d();
    pub fn intr_entry_0x0e();
    pub fn intr_entry_0x0f();
    pub fn intr_entry_0x10();
    pub fn intr_entry_0x11();
    pub fn intr_entry_0x12();
    pub fn intr_entry_0x13();
    pub fn intr_entry_0x14();
    pub fn intr_entry_0x15();
    pub fn intr_entry_0x16();
    pub fn intr_entry_0x17();
    pub fn intr_entry_0x18();
    pub fn intr_entry_0x19();
    pub fn intr_entry_0x1a();
    pub fn intr_entry_0x1b();
    pub fn intr_entry_0x1c();
    pub fn intr_entry_0x1d();
    pub fn intr_entry_0x1e();
    pub fn intr_entry_0x1f();
    pub fn intr_entry_0x20();
    pub fn intr_entry_0x21();
    pub fn intr_entry_0x22();
    pub fn intr_entry_0x23();
    pub fn intr_entry_0x24();
    pub fn intr_entry_0x25();
    pub fn intr_entry_0x26();
    pub fn intr_entry_0x27();
    pub fn intr_entry_0x28();
    pub fn intr_entry_0x29();
    pub fn intr_entry_0x2a();
    pub fn intr_entry_0x2b();
    pub fn intr_entry_0x2c();
    pub fn intr_entry_0x2d();
    pub fn intr_entry_0x2e();
    pub fn intr_entry_0x2f();
    pub fn intr_entry_0x30();

}

lazy_static! {
    static ref IDT_HANDLER_ADDRESS_LIST: [u32; 0x31] = {
        [
            (intr_entry_0x00 as usize).try_into().unwrap(),
            (intr_entry_0x01 as usize).try_into().unwrap(),
            (intr_entry_0x02 as usize).try_into().unwrap(),
            (intr_entry_0x03 as usize).try_into().unwrap(),
            (intr_entry_0x04 as usize).try_into().unwrap(),
            (intr_entry_0x05 as usize).try_into().unwrap(),
            (intr_entry_0x06 as usize).try_into().unwrap(),
            (intr_entry_0x07 as usize).try_into().unwrap(),
            (intr_entry_0x08 as usize).try_into().unwrap(),
            (intr_entry_0x09 as usize).try_into().unwrap(),
            (intr_entry_0x0a as usize).try_into().unwrap(),
            (intr_entry_0x0b as usize).try_into().unwrap(),
            (intr_entry_0x0c as usize).try_into().unwrap(),
            (intr_entry_0x0d as usize).try_into().unwrap(),
            (intr_entry_0x0e as usize).try_into().unwrap(),
            (intr_entry_0x0f as usize).try_into().unwrap(),
            (intr_entry_0x10 as usize).try_into().unwrap(),
            (intr_entry_0x11 as usize).try_into().unwrap(),
            (intr_entry_0x12 as usize).try_into().unwrap(),
            (intr_entry_0x13 as usize).try_into().unwrap(),
            (intr_entry_0x14 as usize).try_into().unwrap(),
            (intr_entry_0x15 as usize).try_into().unwrap(),
            (intr_entry_0x16 as usize).try_into().unwrap(),
            (intr_entry_0x17 as usize).try_into().unwrap(),
            (intr_entry_0x18 as usize).try_into().unwrap(),
            (intr_entry_0x19 as usize).try_into().unwrap(),
            (intr_entry_0x1a as usize).try_into().unwrap(),
            (intr_entry_0x1b as usize).try_into().unwrap(),
            (intr_entry_0x1c as usize).try_into().unwrap(),
            (intr_entry_0x1d as usize).try_into().unwrap(),
            (intr_entry_0x1e as usize).try_into().unwrap(),
            (intr_entry_0x1f as usize).try_into().unwrap(),
            (intr_entry_0x20 as usize).try_into().unwrap(),
            (intr_entry_0x21 as usize).try_into().unwrap(),
            (intr_entry_0x22 as usize).try_into().unwrap(),
            (intr_entry_0x23 as usize).try_into().unwrap(),
            (intr_entry_0x24 as usize).try_into().unwrap(),
            (intr_entry_0x25 as usize).try_into().unwrap(),
            (intr_entry_0x26 as usize).try_into().unwrap(),
            (intr_entry_0x27 as usize).try_into().unwrap(),
            (intr_entry_0x28 as usize).try_into().unwrap(),
            (intr_entry_0x29 as usize).try_into().unwrap(),
            (intr_entry_0x2a as usize).try_into().unwrap(),
            (intr_entry_0x2b as usize).try_into().unwrap(),
            (intr_entry_0x2c as usize).try_into().unwrap(),
            (intr_entry_0x2d as usize).try_into().unwrap(),
            (intr_entry_0x2e as usize).try_into().unwrap(),
            (intr_entry_0x2f as usize).try_into().unwrap(),
            (intr_entry_0x30 as usize).try_into().unwrap(),
        ]
    };
}

fn init_ldt_entry(gate: &mut GateDescriptor, idx: usize) {
    if idx >= IDT_HANDLER_ADDRESS_LIST.len() {
        return
    }

    let address: u32 = (IDT_HANDLER_ADDRESS_LIST[idx] as usize + HIGH_ADDRESS_BASE).try_into().unwrap();
    let mut _gate = GateDescriptor::new(CODE_SELECTOR, address, true, 0, INTR_GATE_ATTR);
    *gate = _gate;
    assert_eq!(CODE_SELECTOR, gate.get_selector());
    assert_eq!(address, gate.get_address());
    assert_eq!(true, gate.is_present());
    assert_eq!(0, gate.get_DPL());
    assert_eq!(INTR_GATE_ATTR, gate.get_attr());
}


pub fn init() {
    extern "C" {
        fn intr_table();
    }

    let idt_table = unsafe {
        core::slice::from_raw_parts_mut(intr_table as usize as *mut GateDescriptor, IDT_MAX_LEN)
    };

    for idx in 0..IDT_MAX_LEN {
        init_ldt_entry(&mut idt_table[idx], idx);
    }
}
