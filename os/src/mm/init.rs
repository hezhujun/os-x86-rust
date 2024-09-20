use alloc::vec;
use core::arch::asm;

use crate::config::*;
use crate::arch::x86::*;
use crate::mm::*;

pub fn init_kernel_page_table() {
    let kernel_pdt_pa = PhysAddr(KERNEL_PDT_PHYS_ADDRESS);
    let kernel_pdt_va = VirtAddr(PDT_VIRT_ADDRESS);
    let kernel_pdt_ppn = kernel_pdt_pa.phys_page_num_floor();
    let kernel_pdt_vpn = kernel_pdt_va.virt_page_num_floor();
    let kernel_pde_array = kernel_pdt_vpn.get_pde_array();
    let kernel_pet_begin_pa = PhysAddr(KERNEL_PDT_PHYS_ADDRESS + MEMORY_PAGE_SIZE);

    // clean #0 pde
    kernel_pde_array[0] = PageDirectoryEntry::empty();

    // #0-#767 pde
    for idx in 0..=767 {
        assert_eq!(kernel_pde_array[idx], PageDirectoryEntry::empty());
    }

    // #768 pde
    {
        let page_pa = kernel_pet_begin_pa;
        let entry = PageDirectoryEntry::new(page_pa.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
        assert_eq!(kernel_pde_array[768].address(), entry.address());
        assert!(kernel_pde_array[768].flag().contains(PdeFlags::P | PdeFlags::RW));
    }

    // #769-#1022 pde
    for idx in 769..1023 {
        let page_pa = PhysAddr(kernel_pet_begin_pa.0 + MEMORY_PAGE_SIZE * (idx - 768));
        let entry = PageDirectoryEntry::new(page_pa.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
        assert_eq!(kernel_pde_array[idx].address(), entry.address());
        assert!(kernel_pde_array[idx].flag().contains(PdeFlags::P | PdeFlags::RW));
    }

    // #1023 pde
    {
        let entry = PageDirectoryEntry::new(kernel_pdt_pa.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
        assert_eq!(kernel_pde_array[1023].address(), entry.address());
        assert!(kernel_pde_array[1023].flag().contains(PdeFlags::P | PdeFlags::RW));
    }

    fn map(vpn: VirtPageNum, ppn: PhysPageNum, isAssert: bool) {
        if isAssert {
            return
        }
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let second_vpn: VirtPageNum = VirtAddr(0x3ff<<22 | index1 << 12).into();
        let pte_array = second_vpn.get_pte_array();
        assert_eq!(pte_array[index2].flag().contains(PteFlags::P), isAssert, "vpn {:#x} ppn {:#x} current address {:#x}", vpn.0, ppn.0, pte_array[index2].address());
        let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), PteFlags::P | PteFlags::RW);
        if isAssert {
            assert_eq!(pte_array[index2].address(), new_entry.address());
            assert!(pte_array[index2].flag().contains(PteFlags::P | PteFlags::RW));
        } else {
            pte_array[index2] = new_entry;
        }
    }

    // map low 1m and kernel space
    let high_ppn_base = VirtAddr(HIGH_ADDRESS_BASE).virt_page_num_floor();
    for idx in 0..(KERNEL_PDT_PHYS_ADDRESS >> 12) {
        let vpn = VirtPageNum(high_ppn_base.0 + idx);
        let ppn = PhysPageNum(idx);
        map(vpn, ppn, true);
    }

    // alloc heap space
    let kernel_heap_pa = PhysAddr(KERNEL_HEAP_PHYS_ADDRESS);
    let kernel_heap_va = VirtAddr(KERNEL_HEAP_VIRT_ADDRESS);
    for idx in 0..KERNEL_HEAP_PAGE_SIZE {
        let page_pa = PhysAddr(kernel_heap_pa.0 + MEMORY_PAGE_SIZE * idx);
        let page_ppn = page_pa.phys_page_num_floor();
        let page_va = VirtAddr(kernel_heap_va.0 + MEMORY_PAGE_SIZE * idx);
        let page_vpn = page_va.virt_page_num_floor();
        map(page_vpn, page_ppn, false);
    }

    // alloc page for phys memory page bitmap
    let phys_frame_bitmap_pa = PhysAddr(PHYS_FRAME_BITMAP_PHYS_ADDRESS);
    let phys_frame_bitmap_va = PhysAddr(PHYS_FRAME_BITMAP_VIRT_ADDRESS);
    for idx in 0..PHYS_FRAME_BITMAP_PAGE_SIZE {
        let page_pa = PhysAddr(phys_frame_bitmap_pa.0 + MEMORY_PAGE_SIZE * idx);
        let page_ppn = page_pa.phys_page_num_floor();
        let page_va = VirtAddr(phys_frame_bitmap_va.0 + MEMORY_PAGE_SIZE * idx);
        let page_vpn = page_va.virt_page_num_floor();
        map(page_vpn, page_ppn, false);
    }

    // alloc page for kernel virt page bitmap
    let kernel_virt_frame_bitmap_pa = PhysAddr(KERNEL_VIRT_FRAME_BITMAP_PHYS_ADDRESS);
    let kernel_virt_frame_bitmap_va = PhysAddr(KERNEL_VIRT_FRAME_BITMAP_VIRT_ADDRESS);
    for idx in 0..KERNEL_VIRT_FRAME_BITMAP_PAGE_SIZE {
        let page_pa = PhysAddr(kernel_virt_frame_bitmap_pa.0 + MEMORY_PAGE_SIZE * idx);
        let page_ppn = page_pa.phys_page_num_floor();
        let page_va = VirtAddr(kernel_virt_frame_bitmap_va.0 + MEMORY_PAGE_SIZE * idx);
        let page_vpn = page_va.virt_page_num_floor();
        map(page_vpn, page_ppn, false);
    }
}
