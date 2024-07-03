pub const MEMORY_PAGE_SIZE: usize = 0x1000;
pub const PTE_SIZE_IN_PAGE: usize = 0x1000 / 4;
pub const KERNEL_HEAP_PAGE_SIZE: usize = 0x50;
pub const HIGH_ADDRESS_BASE: usize = 0xc0000000;

pub const RPL0: u8 = 0b00;
pub const RPL1: u8 = 0b01;
pub const RPL2: u8 = 0b10;
pub const RPL3: u8 = 0b11;
pub const TI_GDT: u8 = 0b0;
pub const TI_LDT: u8 = 0b1;
pub const CODE_SELECTOR: u16 = (1u16 << 3) | ((TI_GDT as u16) << 2) | RPL0 as u16;
pub const DATA_SELECTOR: u16 = (2u16 << 3) | ((TI_GDT as u16) << 2) | RPL0 as u16;
