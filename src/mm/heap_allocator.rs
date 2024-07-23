use buddy_system_allocator::LockedHeap;
use crate::config::*;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(KERNEL_HEAP_VIRT_ADDRESS, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
