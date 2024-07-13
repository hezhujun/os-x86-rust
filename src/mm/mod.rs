pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod page_table;
pub mod memory_set;
mod init;

use core::assert;
use crate::arch::x86::AddressRangeDescriptorStructure;

pub use address::*;
pub use frame_allocator::*;
pub use memory_set::*;
pub use page_table::*;

const ARDS_MAX_COUNT: usize = 25;
static mut FRAME_BEGIN_PHYS_ADDRESS: usize = 0;
static mut MEMROY_PAGE_BITMAP_VIRT_ADDRESS: usize = 0;
static mut MEMROY_PAGE_BITMAP_PAGE_SIZE: usize = 0;
pub const KERNEL_PDT_PHYS_ADDRESS: usize = 0x900000;
pub const KERNEL_PDT_VIRT_ADDRESS: usize = 0xc0900000;

pub fn set_frame_begin_phys_address(address: usize) {
    unsafe {
        FRAME_BEGIN_PHYS_ADDRESS = address;
    }
}

pub fn set_memory_page_bitmap_info(address: usize, page_size: usize) {
    unsafe {
        MEMROY_PAGE_BITMAP_VIRT_ADDRESS = address;
        MEMROY_PAGE_BITMAP_PAGE_SIZE = page_size;
    }
}

lazy_static! {
    static ref ARDS_COUNT: usize = {
        let ards_len = unsafe {
            (0xc0090200 as *const u32).as_ref().unwrap()
        };
        *ards_len as usize
    };

    static ref ARDS_ARRAY: [AddressRangeDescriptorStructure; ARDS_MAX_COUNT] = {
        let count: usize = *ARDS_COUNT;
        assert!(count <= ARDS_MAX_COUNT);
        let old_ards = unsafe {
            core::slice::from_raw_parts((0xc0090200usize + 4) as *const AddressRangeDescriptorStructure, count)
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
            assert!(ards_address_begin + unsafe { FRAME_BEGIN_PHYS_ADDRESS as u64 } < ards_address_end);
            let ards_address_begin: usize = (ards_address_begin + unsafe { FRAME_BEGIN_PHYS_ADDRESS as u64 }).try_into().unwrap();
            let ards_address_end: usize = ards_address_end.try_into().unwrap();
            let ards_address_begin = PhysAddr(ards_address_begin);
            let ards_address_end = PhysAddr(ards_address_end);
            (ards_address_begin.phys_page_num_ceil(), ards_address_end.phys_page_num_floor())
        } else {
            panic!("no free memory");
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
    init::memory_init();
    info!("mm::init done");
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
    
    MEMORY_INFO.ards_array.iter().enumerate().for_each(|(idx, ards)| {
        let address_begin = ards.get_addr();
        let address_size = ards.get_length();
        let address_end = address_begin + address_size;
        info!("ards #{} [{:#x},{:#x}) size {:#x} type {}", idx, address_begin, address_end, address_size, ards.memory_type);
    });
}

