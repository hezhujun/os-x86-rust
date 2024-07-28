use alloc::sync::{Arc, Weak};
use spin::Mutex;

use crate::{intr::IntrContext, mm::MapArea};
use crate::config::*;
use crate::mm::{alloc_kernel_virt_frame, MapPermission, VirtAddr, VirtPageNum};
use crate::utils::*;
use super::{context::TaskContext, process::ProcessControlBlock};

pub enum TaskStatus {
    Ready, Running, Block
}

pub struct TaskControlBlockInner {
    pub status: TaskStatus,
    pub intr_cx: IntrContext,
    pub task_cx: TaskContext,
    pub kernel_stack_top_address: VirtAddr,
    pub user_stack_top_address: Option<VirtAddr>,
    pub kernel_stack_map_area: MapArea,
    pub user_stack_map_area: Option<MapArea>,
}

pub struct TaskControlBlock {
    pub tid: usize,
    pub process: Weak<ProcessControlBlock>,
    pub task_inner: Arc<Mutex<TaskControlBlockInner>>,
}

impl TaskControlBlock {
    pub fn new(process: Arc<ProcessControlBlock>, entry_point: usize, is_kernel_task: bool) -> Self {
        let mut process_inner = process.inner.lock();
        let tid = process_inner.tid_allocator.alloc().unwrap();
        // user stack
        let (user_stack_top_va, user_stack_area, intr_context) = if !is_kernel_task {
            let user_stack_top_address = USER_STACK_TOP_VIRT_ADDRESS - (USER_STACK_SIZE + MEMORY_PAGE_SIZE) * tid;
            let user_stack_bottom_address = user_stack_top_address - USER_STACK_SIZE;
            let user_stack_top_va = VirtAddr(user_stack_top_address);
            let user_stack_bottom_va = VirtAddr(user_stack_bottom_address);
            let mut user_stack_area = MapArea::new(
                VirtPageNum::from(user_stack_bottom_va)..VirtPageNum::from(user_stack_top_va),
                None,
                MapPermission::R | MapPermission::W | MapPermission::U
            );
            user_stack_area.map(&mut process_inner.memory_set.page_table);
            (Some(user_stack_top_va), Some(user_stack_area), IntrContext::user_intr_context(VirtAddr(entry_point), VirtAddr(user_stack_top_address)))
        } else {
            (None, None, IntrContext::kernel_intr_context(VirtAddr(entry_point)))
        };
        
        // kernel stack
        let kernel_stack_vstub = alloc_kernel_virt_frame(KERNEL_STACK_PAGE_SIZE + 1).unwrap();
        let kernel_stack_bottom_vpn = VirtPageNum(kernel_stack_vstub.base_vpn.0 + 1);
        let kernel_stack_top_vpn = VirtPageNum(kernel_stack_vstub.base_vpn.0 + kernel_stack_vstub.len);
        let mut kernel_stack_area = MapArea::new(
            kernel_stack_bottom_vpn..kernel_stack_top_vpn, 
            Some(kernel_stack_vstub), 
            MapPermission::R | MapPermission::W
        );
        kernel_stack_area.map(&mut process_inner.memory_set.page_table);
        let task_inner = TaskControlBlockInner {
            status: TaskStatus::Ready,
            intr_cx: IntrContext::empty(),
            task_cx: TaskContext::go_to_intr_return(kernel_stack_top_vpn.base_address(), intr_context),
            kernel_stack_top_address: kernel_stack_top_vpn.base_address(),
            user_stack_top_address: user_stack_top_va,
            kernel_stack_map_area: kernel_stack_area,
            user_stack_map_area: user_stack_area,
        };

        Self { tid: tid, process: Arc::downgrade(&process), task_inner: Arc::new(Mutex::new(task_inner)) }
    }
}

impl Drop for TaskControlBlock {
    fn drop(&mut self) {
        if let Some(process) = self.process.upgrade() {
            let mut process_inner = process.inner.lock();
            let mut task_inner = self.task_inner.lock();
            if let Some(mut map_area) = task_inner.user_stack_map_area.take() {
                map_area.unmap(&mut process_inner.memory_set.page_table)
            }
            task_inner.kernel_stack_map_area.unmap(&mut process_inner.memory_set.page_table);
            process_inner.tid_allocator.dealloc(self.tid);
        }
    }
}

pub type ThreadIdAllocator = IdAllocator<THREAD_ID_BITMAP_SIZE>;

pub fn create_thread_id_allocator() -> ThreadIdAllocator {
    IdAllocator::new(Bitmap::<THREAD_ID_BITMAP_SIZE>::new([0; THREAD_ID_BITMAP_SIZE]), 0, 0, THREAD_MAX_ID, 0)
}
