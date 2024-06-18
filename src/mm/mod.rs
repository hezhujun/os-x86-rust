use crate::arch::x86::AddressRangeDescriptorStructure;


pub fn memory_info() {
    let ards_len = unsafe {
        (0x90200 as *const u32).as_ref().unwrap()
    };
    println!("ards len: {}", ards_len);
    let ards_array = unsafe {
        core::slice::from_raw_parts((0x90200+4) as *const AddressRangeDescriptorStructure, *ards_len as usize)
    };
    ards_array.iter().enumerate().for_each(|(idx, ards)| {
        let address_begin = ards.get_addr();
        let address_size = ards.get_length();
        let address_end = address_begin + address_size;
        println!("ards #{} [{:#x},{:#x}) size {:#x} type {}", idx, address_begin, address_end, address_size, ards.memory_type);
    });
    let usable_size = ards_array.iter().filter_map(|ards| {
        if ards.is_usable() {
            Some(ards.get_length())
        } else {
            None
        }
    }).fold(0, |acc, size| acc + size);
    println!("usable memory size {:#x}", usable_size);
}

