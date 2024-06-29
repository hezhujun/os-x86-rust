use alloc::vec;
use core::arch::asm;

use crate::config::*;
use crate::arch::x86::*;
use crate::mm::address::*;
use super::set_frame_begin_address;
use crate::mm::heap_allocator;

pub fn memory_init() {
    let gdtr = 0x90408 as *mut GDTRegister;
    unsafe {
        (*gdtr).set(511, 0xc0090000);
    }
    let (kernel_heap_base_pa, end_page_pa) = init_kernel_page_table();
    unsafe {
        asm!("lgdt [0x90408]");
    }
    set_frame_begin_address(end_page_pa.0);
    let kernel_heap_base_address = kernel_heap_base_pa.0 + HIGH_ADDRESS_BASE;
    let kernel_heap_end_address = kernel_heap_base_address + KERNEL_HEAP_PAGE_SIZE * MEMORY_PAGE_SIZE;
    heap_allocator::init(kernel_heap_base_address);

    info!("kernel heap space [{:#x}, {:#x})", kernel_heap_base_address, kernel_heap_end_address);
}

/// return (kernel_heap_base_phys_address, end_page_phys_address)
fn init_kernel_page_table() -> (PhysAddr, PhysAddr) {
    let root_directory_page_address = PhysAddr(0x100000);
    let root = root_directory_page_address.phys_page_num_floor();
    root.get_bytes_array().iter_mut().for_each(|b| { *b = 0 });
    let root_pde_array = root.get_pde_arrray();
    let page_base_address = root_directory_page_address.0 + MEMORY_PAGE_SIZE;
    let mut end_page_address = page_base_address;

    // #0 #768 pde
    {
        let page_address = PhysAddr(end_page_address);
        end_page_address += MEMORY_PAGE_SIZE;
        let page_ppn = page_address.phys_page_num_floor();
        page_ppn.get_bytes_array().iter_mut().for_each(|b| { *b = 0 });
        let entry = PageDirectoryEntry::new(page_address.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW | PdeFlags::US);
        root_pde_array[0] = entry;
        root_pde_array[768] = entry;
    }

    // #769-#1023 pde
    for idx in 769..1024 {
        let page_address = PhysAddr(end_page_address);
        end_page_address += MEMORY_PAGE_SIZE;
        let page_ppn = page_address.phys_page_num_floor();
        page_ppn.get_bytes_array().iter_mut().for_each(|b| { *b = 0 });
        let entry = PageDirectoryEntry::new(page_address.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW | PdeFlags::US);
        root_pde_array[idx] = entry;
    }

    fn map(root: PhysPageNum, vpn: VirtPageNum, ppn: PhysPageNum) {
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let pde_array = root.get_pde_arrray();
        let second_ppn: PhysPageNum = pde_array[index1].into();
        let pte_array = second_ppn.get_pte_arrray();
        assert_eq!(pte_array[index2].flag().contains(PteFlags::P), false);
        let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), PteFlags::P | PteFlags::RW);
        pte_array[index2] = new_entry;
    }

    // map low 1m
    for idx in 0..root.0 {
        let vpn = VirtPageNum(idx);
        let ppn = PhysPageNum(idx);
        map(root, vpn, ppn)
    }

    // alloc heap space
    let kernel_heap_base_address = end_page_address;
    for _ in 0..KERNEL_HEAP_PAGE_SIZE {
        let page_address = PhysAddr(end_page_address);
        end_page_address += MEMORY_PAGE_SIZE;
        let page_ppn = page_address.phys_page_num_floor();
        page_ppn.get_bytes_array().iter_mut().for_each(|b| { *b = 0 });
    }

    // map 0x100000 - end_page_address
    let high_ppn_base = VirtAddr(HIGH_ADDRESS_BASE).virt_page_num_floor();
    let begin_ppn = root;
    let end_ppn = VirtAddr(end_page_address).virt_page_num_floor();
    for idx in begin_ppn.0..end_ppn.0 {
        map(root, VirtPageNum(high_ppn_base.0 + idx), PhysPageNum(idx));
    }

    let cr3 = Cr3::new(root_directory_page_address.0.try_into().unwrap(), PdbrFlag::empty());
    cr3.write();
    let mut cr0 = Cr0::read();
    cr0 |= Cr0::PG;
    cr0.write();

    (PhysAddr(kernel_heap_base_address), PhysAddr(end_page_address))
}
