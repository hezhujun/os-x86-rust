use core::assert;
use crate::arch::x86::AddressRangeDescriptorStructure;
use crate::arch::x86::SegmentDescriptor;

const ARDS_MAX_COUNT: usize = 25;

lazy_static! {
    static ref GDT: [SegmentDescriptor; 64] = {
        let old_gdt = unsafe {
            core::slice::from_raw_parts((0x90000) as *const SegmentDescriptor, 64)
        };
        let mut gdt = [SegmentDescriptor::empty(); 64];
        for idx in 0..64 {
            gdt[idx] = old_gdt[idx];
        }
        gdt
    };

    static ref ARDS_COUNT: usize = {
        let ards_len = unsafe {
            (0x90200 as *const u32).as_ref().unwrap()
        };
        *ards_len as usize
    };

    static ref ARDS_ARRAY: [AddressRangeDescriptorStructure; ARDS_MAX_COUNT] = {
        let count: usize = *ARDS_COUNT;
        assert!(count <= ARDS_MAX_COUNT);
        let old_ards = unsafe {
            core::slice::from_raw_parts((0x90200+4) as *const AddressRangeDescriptorStructure, count)
        };
        let mut ards = [AddressRangeDescriptorStructure::empty(); 25];
        for idx in 0..count {
            ards[idx] = old_ards[idx];
        }
        ards
    };

    static ref ARDS_ARRAY_REFERENCE: &'static [AddressRangeDescriptorStructure] = {
        &ARDS_ARRAY[0..*ARDS_COUNT]
    };
}


pub fn init() {
    memory_info();
}

fn memory_info() {
    extern "C" {
        fn skernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn ekernel();
    }
    info!("kernel memory info");
    info!("text   [{:#x}, {:#x})", stext as usize, etext as usize);
    info!("rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!("data   [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!("bss    [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!("total  [{:#x}, {:#x})", skernel as usize, ekernel as usize);
    
    ARDS_ARRAY_REFERENCE.iter().enumerate().for_each(|(idx, ards)| {
        let address_begin = ards.get_addr();
        let address_size = ards.get_length();
        let address_end = address_begin + address_size;
        info!("ards #{} [{:#x},{:#x}) size {:#x} type {}", idx, address_begin, address_end, address_size, ards.memory_type);
    });
}

