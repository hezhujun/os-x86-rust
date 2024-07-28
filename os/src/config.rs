pub const MEMORY_PAGE_SIZE: usize = 0x1000;
pub const PTE_SIZE_IN_PAGE: usize = 0x1000 / 4;

// kerenl 开始物理地址
pub const KERNEL_BEGIN_PHYS_ADDRESS: usize = 0x100000;
pub const KERNEL_LOAD_DATA_SIZE: usize = 0x800000;
// page table 开始物理地址
pub const KERNEL_PDT_PHYS_ADDRESS: usize = KERNEL_BEGIN_PHYS_ADDRESS + KERNEL_LOAD_DATA_SIZE;
pub const KERNEL_PAGE_TABLE_DATA_SIZE: usize = 0x100000;
// 内核堆开始物理地址
pub const KERNEL_HEAP_PHYS_ADDRESS: usize = KERNEL_PDT_PHYS_ADDRESS + KERNEL_PAGE_TABLE_DATA_SIZE;
pub const KERNEL_HEAP_PAGE_SIZE: usize = 0x500;
pub const KERNEL_HEAP_SIZE: usize = KERNEL_HEAP_PAGE_SIZE * MEMORY_PAGE_SIZE;
// 物理帧 bitmap 开始物理地址
pub const PHYS_FRAME_BITMAP_PHYS_ADDRESS: usize = KERNEL_HEAP_PHYS_ADDRESS + KERNEL_HEAP_SIZE;
pub const PHYS_FRAME_BITMAP_SIZE: usize = 0x100000 / 8;  // 4G / MEMORY_PAGE_SIZE / 8
pub const PHYS_FRAME_BITMAP_PAGE_SIZE: usize = PHYS_FRAME_BITMAP_SIZE / MEMORY_PAGE_SIZE;
// 内核虚拟内存空间 bitmap 开始的物理地址
pub const KERNEL_VIRT_FRAME_BITMAP_PHYS_ADDRESS: usize = PHYS_FRAME_BITMAP_PHYS_ADDRESS + PHYS_FRAME_BITMAP_SIZE;
pub const KERNEL_VIRT_FRAME_BITMAP_SIZE: usize = PHYS_FRAME_BITMAP_SIZE / 4;  // 1G / MEMORY_PAGE_SIZE / 8
pub const KERNEL_VIRT_FRAME_BITMAP_PAGE_SIZE: usize = KERNEL_VIRT_FRAME_BITMAP_SIZE / MEMORY_PAGE_SIZE;
// 可用物理帧开始地址
pub const FREE_PHYS_FRAME_BEGIN_ADDRESS: usize = KERNEL_VIRT_FRAME_BITMAP_PHYS_ADDRESS + KERNEL_VIRT_FRAME_BITMAP_SIZE;


// kerenl 开始虚拟地址
pub const KERNEL_BEGIN_VIRT_ADDRESS: usize = 0xc0100000;
// 内核堆开始虚拟地址
pub const KERNEL_HEAP_VIRT_ADDRESS: usize = KERNEL_BEGIN_VIRT_ADDRESS + KERNEL_LOAD_DATA_SIZE;
// 物理帧 bitmap 开始虚拟地址
pub const PHYS_FRAME_BITMAP_VIRT_ADDRESS: usize = KERNEL_HEAP_VIRT_ADDRESS + KERNEL_HEAP_SIZE;
// 内核虚拟内存空间 bitmap 开始的物理地址
pub const KERNEL_VIRT_FRAME_BITMAP_VIRT_ADDRESS: usize = PHYS_FRAME_BITMAP_VIRT_ADDRESS + PHYS_FRAME_BITMAP_SIZE;
// 可用内核虚拟帧开始地址
pub const FREE_KERNEL_VIRT_FRAME_BEGIN_ADDRESS: usize = KERNEL_VIRT_FRAME_BITMAP_VIRT_ADDRESS + KERNEL_VIRT_FRAME_BITMAP_SIZE;
// 可用内核虚拟帧结束地址
pub const FREE_KERNEL_VIRT_FRAME_END_ADDRESS: usize = 0xffc00000;
// page directory table 虚拟地址
pub const PDT_VIRT_ADDRESS: usize = 0xfffff000;

pub const KERNEL_ORIGIN_STACK_TOP_VIRT_ADDRESS: usize = 0xc0090000;

pub const THREAD_KERNEL_STACK_SIZE: usize = 0x10000;
pub const THREAD_USER_STACK_SIZE: usize = 0x10000;

pub const PROCESS_MAX_ID: usize = 1000 * 8;
pub const PROCESS_ID_BITMAP_SIZE: usize = 1000;
pub const THREAD_MAX_ID: usize = 1000 * 8;
pub const THREAD_ID_BITMAP_SIZE: usize = 1000;

pub const HIGH_ADDRESS_BASE: usize = 0xc0000000;
pub const KERNEL_STACK_TOP_VIRT_ADDRESS: usize = 0xffc00000;
pub const KERNEL_STACK_SIZE: usize = 0x10000;
pub const KERNEL_STACK_PAGE_SIZE: usize = KERNEL_STACK_SIZE >> 12;
pub const USER_STACK_TOP_VIRT_ADDRESS: usize = 0xc0000000;
pub const USER_STACK_SIZE: usize = 0x10000;

pub const GDT_SIZE: usize = 512 / 8;
pub const RPL0: u8 = 0b00;
pub const RPL1: u8 = 0b01;
pub const RPL2: u8 = 0b10;
pub const RPL3: u8 = 0b11;
pub const TI_GDT: u8 = 0b0;
pub const TI_LDT: u8 = 0b1;
pub const CODE_SELECTOR: u16 = (1u16 << 3) | ((TI_GDT as u16) << 2) | RPL0 as u16;
pub const DATA_SELECTOR: u16 = (2u16 << 3) | ((TI_GDT as u16) << 2) | RPL0 as u16;
pub const USER_CODE_SELECTOR: u16 = (3u16 << 3) | ((TI_GDT as u16) << 2) | RPL3 as u16;
pub const USER_DATA_SELECTOR: u16 = (4u16 << 3) | ((TI_GDT as u16) << 2) | RPL3 as u16;
pub const TSS_SELECTOR: u16 = (5u16 << 3) | ((TI_GDT as u16) << 2) | RPL0 as u16;
