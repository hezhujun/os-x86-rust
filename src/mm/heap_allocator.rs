use buddy_system_allocator::LockedHeap;
use crate::config::{KERNEL_HEAP_PAGE_SIZE, MEMORY_PAGE_SIZE};

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

const HEAP_SPACE_LEN: usize = KERNEL_HEAP_PAGE_SIZE * MEMORY_PAGE_SIZE;

pub fn init(heap_base_address: usize) {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(heap_base_address, HEAP_SPACE_LEN);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
