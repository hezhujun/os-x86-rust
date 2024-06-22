pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;

use core::assert;
use crate::arch::x86::AddressRangeDescriptorStructure;
use crate::arch::x86::SegmentDescriptor;

pub use address::*;
pub use frame_allocator::*;
pub use heap_allocator::*;

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

pub struct MemoryInfo<'a> {
    pub kernel_space: (usize, usize),
    pub stack_space: (usize, usize),
    pub ards_array: &'a [AddressRangeDescriptorStructure]
}

impl<'a> MemoryInfo<'a> {
    fn new(kernel_space: (usize, usize), stack_space: (usize, usize), ards_array: &'a [AddressRangeDescriptorStructure]) -> Self {
        Self { kernel_space, stack_space, ards_array }
    }

    pub fn get_frame_space_range(&self) -> (PhysPageNum, PhysPageNum) {
        let mut max_memory_size: u64 = 0;
        let mut useable_ards: Option<&AddressRangeDescriptorStructure> = None;
        self.ards_array.iter().for_each(|ards| {
            if ards.get_length() > max_memory_size {
                max_memory_size = ards.get_length();
                useable_ards = Some(ards);
            }
        });
        assert!(useable_ards.is_some());
        if let Some(ards) = useable_ards {
            let kernel_address_begin = self.kernel_space.0 as u64;
            let kernel_address_end = self.kernel_space.1 as u64;
            let ards_address_begin = ards.get_addr();
            let ards_address_end = ards.get_addr() + ards.get_length();
            if kernel_address_begin <= ards_address_begin && ards_address_begin < kernel_address_end {
                assert!(false, "no free memory");
            }
            if ards_address_begin <= kernel_address_end && kernel_address_end < ards_address_end {
                assert!(false, "no free memory");
            }
            let ards_address_begin = PhysAddr(ards_address_begin.try_into().unwrap());
            let ards_address_end = PhysAddr(ards_address_end.try_into().unwrap());
            (ards_address_begin.phys_page_num_ceil(), ards_address_end.phys_page_num_floor())
        } else {
            (PhysAddr(0x100000).phys_page_num_ceil(), PhysAddr(0x100000).phys_page_num_floor())
        }
    }
}

lazy_static! {
    static ref MEMORY_INFO: MemoryInfo<'static> = {
        extern "C" {
            fn skernel();
            fn sbss_with_stack();
            fn ebss_with_stack();
            fn ekernel();
        }
        let kernel_space = (skernel as usize, ekernel as usize);
        let stack_space = (sbss_with_stack as usize, ebss_with_stack as usize);
        MemoryInfo::new(kernel_space, stack_space, &ARDS_ARRAY_REFERENCE)
    };
}


pub fn init() {
    memory_info();
    heap_allocator::init();
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
        fn sbss_with_stack();
        fn ebss_with_stack();
        fn sbss();
        fn ebss();
        fn ekernel();
    }
    info!("kernel memory info");
    info!("text   [{:#x}, {:#x})", stext as usize, etext as usize);
    info!("rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!("data   [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!("stack  [{:#x}, {:#x})", sbss_with_stack as usize, ebss_with_stack as usize);
    info!("bss    [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!("total  [{:#x}, {:#x})", skernel as usize, ekernel as usize);

    assert!(ekernel as usize <= 0x9fc00);
    
    MEMORY_INFO.ards_array.iter().enumerate().for_each(|(idx, ards)| {
        let address_begin = ards.get_addr();
        let address_size = ards.get_length();
        let address_end = address_begin + address_size;
        info!("ards #{} [{:#x},{:#x}) size {:#x} type {}", idx, address_begin, address_end, address_size, ards.memory_type);
    });
}

