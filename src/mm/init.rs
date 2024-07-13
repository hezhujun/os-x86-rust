use alloc::vec;
use core::arch::asm;

use crate::config::*;
use crate::arch::x86::*;
use crate::mm::*;
use crate::mm::heap_allocator;

pub fn memory_init() {
    let (kernel_heap_base_pa, memory_page_bitmap_base_va, memory_page_bitmap_page_size, end_page_pa) = init_kernel_page_table();
    set_frame_begin_phys_address(end_page_pa.0);
    set_memory_page_bitmap_info(memory_page_bitmap_base_va.0, memory_page_bitmap_page_size);
    let kernel_heap_base_pa = kernel_heap_base_pa.0 + HIGH_ADDRESS_BASE;
    let kernel_heap_end_pa = kernel_heap_base_pa + KERNEL_HEAP_PAGE_SIZE * MEMORY_PAGE_SIZE;
    let kernel_heap_base_va = KERNEL_HEAP_BASE_VIRT_ADDRESS;
    let kernel_heap_end_va = kernel_heap_base_va + KERNEL_HEAP_PAGE_SIZE * MEMORY_PAGE_SIZE;
    heap_allocator::init();
    info!("kernel heap space in phys address [{:#x}, {:#x})", kernel_heap_base_pa, kernel_heap_end_pa);
    info!("kernel heap space in phys address [{:#x}, {:#x})", kernel_heap_base_va, kernel_heap_end_va);
}

/// return (kernel_heap_base_phys_address, memory_page_bitmap_base_virt_address, memory_page_bitmap_page_size, memory end_page_phys_address)
fn init_kernel_page_table() -> (PhysAddr, VirtAddr, usize, PhysAddr) {
    let root_directory_page_pa = PhysAddr(KERNEL_PDT_PHYS_ADDRESS);
    let root_directory_page_va = VirtAddr(PAGE_TABLE_VIRT_ADDRESS);
    let root_ppn = root_directory_page_pa.phys_page_num_floor();
    let root_vpn = root_directory_page_va.virt_page_num_floor();
    let root_pde_array = root_vpn.get_pde_arrray();
    let page_base_pa = root_directory_page_pa.0 + MEMORY_PAGE_SIZE;
    let mut end_page_pa = page_base_pa;

    // // #0-#767 pde
    // for idx in 0..=767 {
    //     assert_eq!(root_pde_array[idx], PageDirectoryEntry::empty());
    // }

    // #768 pde
    {
        // let page_pa: usize = end_page_pa;
        end_page_pa += MEMORY_PAGE_SIZE;
        // let entry = PageDirectoryEntry::new(page_pa.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
        // assert_eq!(root_pde_array[768].address(), entry.address());
        // assert!(root_pde_array[768].flag().contains(PdeFlags::P | PdeFlags::RW));
    }

    // #769-#1022 pde
    // for idx in 769..1023 {
    //     let page_address = PhysAddr(end_page_pa + MEMORY_PAGE_SIZE * (idx - 769));
    //     let entry = PageDirectoryEntry::new(page_address.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
    //     assert_eq!(root_pde_array[idx].address(), entry.address());
    //     assert!(root_pde_array[idx].flag().contains(PdeFlags::P | PdeFlags::RW));
    // }
    end_page_pa += MEMORY_PAGE_SIZE * (1023 - 769);

    // #1023 pde
    // {
    //     let entry = PageDirectoryEntry::new(root_directory_page_pa.0.try_into().unwrap(), PdeFlags::P | PdeFlags::RW);
    //     assert_eq!(root_pde_array[1023].address(), entry.address());
    //     assert!(root_pde_array[1023].flag().contains(PdeFlags::P | PdeFlags::RW));
    // }

    fn map(vpn: VirtPageNum, ppn: PhysPageNum, isAssert: bool) {
        if isAssert {
            return
        }
        let index2 = vpn.0 & 0x3ff;
        let index1 = (vpn.0 >> 10)& 0x3ff;
        let root_va = VirtAddr(PAGE_TABLE_VIRT_ADDRESS);
        let root_vpn = root_va.virt_page_num_floor();
        let pde_array = root_vpn.get_pde_arrray();
        let second_vpn: VirtPageNum = VirtAddr(0x3ff<<22 | index1 << 12).into();
        let pte_array = second_vpn.get_pte_arrray();
        assert_eq!(pte_array[index2].flag().contains(PteFlags::P), isAssert, "vpn {:#x} ppn {:#x} current address {:#x}", vpn.0, ppn.0, pte_array[index2].address());
        let new_entry = PageTableEntry::new(ppn.base_address().0.try_into().unwrap(), PteFlags::P | PteFlags::RW);
        if isAssert {
            assert_eq!(pte_array[index2].address(), new_entry.address());
            assert!(pte_array[index2].flag().contains(PteFlags::P | PteFlags::RW));
        } else {
            pte_array[index2] = new_entry;
        }
    }

    // // map low 1m and kernel space
    // let high_ppn_base = VirtAddr(HIGH_ADDRESS_BASE).virt_page_num_floor();
    // for idx in 0..(KERNEL_PDT_PHYS_ADDRESS >> 12) {
    //     let vpn = VirtPageNum(high_ppn_base.0 + idx);
    //     let ppn = PhysPageNum(idx);
    //     map(vpn, ppn, true);
    // }

    // alloc heap space
    let kernel_heap_base_pa = end_page_pa;
    let kernel_heap_base_va = KERNEL_HEAP_BASE_VIRT_ADDRESS;
    end_page_pa += MEMORY_PAGE_SIZE * KERNEL_HEAP_PAGE_SIZE;
    let high_ppn_base = VirtAddr(HIGH_ADDRESS_BASE).virt_page_num_floor();
    let begin_ppn = VirtAddr(kernel_heap_base_pa).virt_page_num_floor();
    let end_ppn = VirtAddr(end_page_pa).virt_page_num_floor();
    for idx in 0..KERNEL_HEAP_PAGE_SIZE {
        let page_pa = PhysAddr(kernel_heap_base_pa + MEMORY_PAGE_SIZE * idx);
        let page_ppn = page_pa.phys_page_num_floor();
        let page_va = VirtAddr(kernel_heap_base_va + MEMORY_PAGE_SIZE * idx);
        let page_vpn = page_va.virt_page_num_floor();
        map(page_vpn, page_ppn, false);
        page_vpn.get_bytes_array().iter_mut().for_each(|b| { *b = 0 });
    }

    // alloc page for memory page bitmap
    let page_needed = (PAGE_BITMAP_LEN_IN_4G + MEMORY_PAGE_SIZE - 1) / MEMORY_PAGE_SIZE; // value is 32
    debug!("page needed {}", page_needed);
    let memory_page_bitmap_base_pa = kernel_heap_base_pa + MEMORY_PAGE_SIZE * KERNEL_HEAP_PAGE_SIZE;
    let memory_page_bitmap_base_va = kernel_heap_base_va + MEMORY_PAGE_SIZE * KERNEL_HEAP_PAGE_SIZE;
    end_page_pa += MEMORY_PAGE_SIZE * page_needed;
    let high_ppn_base = VirtAddr(HIGH_ADDRESS_BASE).virt_page_num_floor();
    let begin_ppn = VirtAddr(memory_page_bitmap_base_pa).virt_page_num_floor();
    let end_ppn = VirtAddr(end_page_pa).virt_page_num_floor();
    for idx in 0..page_needed {
        let page_pa = PhysAddr(memory_page_bitmap_base_pa + MEMORY_PAGE_SIZE * idx);
        let page_ppn = page_pa.phys_page_num_floor();
        let page_va = VirtAddr(memory_page_bitmap_base_va + MEMORY_PAGE_SIZE * idx);
        let page_vpn = page_va.virt_page_num_floor();
        map(page_vpn, page_ppn, false);
        page_vpn.get_bytes_array().iter_mut().for_each(|b| { *b = 0 });
    }

    (PhysAddr(kernel_heap_base_pa), VirtAddr(memory_page_bitmap_base_va), page_needed, PhysAddr(end_page_pa))
}
