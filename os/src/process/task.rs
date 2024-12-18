use core::iter::zip;

use alloc::sync::{Arc, Weak};
use spin::Mutex;

use crate::{intr::IntrContext, mm::MapArea};
use crate::config::*;
use crate::mm::*;
use crate::utils::*;
use super::ProcessControlBlockInner;
use super::{context::TaskContext, process::ProcessControlBlock};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskStatus {
    Ready, Running, Block
}

pub struct TaskControlBlockInner {
    pub status: TaskStatus,
    pub intr_cx: IntrContext,
    pub task_cx: TaskContext,
    kernel_stack_vstub: VirtFrameStub,
    pub kernel_stack_top_address: VirtAddr,
    pub user_stack_top_address: Option<VirtAddr>,
    pub kernel_stack_map_area: MapArea,
    pub user_stack_map_area: Option<MapArea>,
    pub exit_code: Option<isize>,
}

impl TaskControlBlockInner {
    pub fn repair_page_fault(&mut self, page_table: &mut PageTable) -> bool {
        let mut is_modified = false;
        if let Some(user_stack_map_area) = self.user_stack_map_area.as_mut() {
            is_modified |= user_stack_map_area.map_if_need(page_table);

            if !is_modified {
                // 已经创建过页表，判断进程是否是有过 fork 操作
                is_modified |= user_stack_map_area.copy_if_need(page_table);
            }
        }
        
        assert!(is_modified);

        is_modified
    }
}

pub struct TaskControlBlock {
    pub tid: usize,
    pub process: Weak<ProcessControlBlock>,
    pub inner: Arc<Mutex<TaskControlBlockInner>>,
}

impl TaskControlBlock {
    pub fn new<T>(process: Arc<ProcessControlBlock>, entry_point: usize, is_kernel_task: bool, user_data: Option<T>) -> Self {
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
                MapPermission::R | MapPermission::W | MapPermission::U
            );
            let mut intr_cx = IntrContext::user_intr_context(VirtAddr(entry_point), VirtAddr(user_stack_top_address));
            if let Some(use_data) = user_data {
                user_stack_area.map_if_need(&mut process_inner.memory_set.page_table);
                Self::push_data_to_user_stack(&mut intr_cx, use_data);
            }
            // user_stack_area.map_if_need(&mut process_inner.memory_set.page_table);
            (Some(user_stack_top_va), Some(user_stack_area), intr_cx)
        } else {
            (None, None, IntrContext::kernel_intr_context(VirtAddr(entry_point)))
        };
        
        // kernel stack
        let kernel_stack_vstub = alloc_kernel_virt_frame(KERNEL_STACK_PAGE_SIZE + 2).unwrap();
        let kernel_stack_bottom_vpn = VirtPageNum(kernel_stack_vstub.base_vpn.0 + 1);
        let kernel_stack_top_vpn = VirtPageNum(kernel_stack_bottom_vpn.0 + KERNEL_STACK_PAGE_SIZE);
        let mut kernel_stack_area = MapArea::new(
            kernel_stack_bottom_vpn..kernel_stack_top_vpn, 
            MapPermission::R | MapPermission::W
        );
        kernel_stack_area.map_if_need(&mut process_inner.memory_set.page_table);
        let task_inner = TaskControlBlockInner {
            status: TaskStatus::Ready,
            intr_cx: intr_context,
            task_cx: TaskContext::go_to_intr_return(kernel_stack_top_vpn.base_address(), intr_context),
            kernel_stack_vstub,
            kernel_stack_top_address: kernel_stack_top_vpn.base_address(),
            user_stack_top_address: user_stack_top_va,
            kernel_stack_map_area: kernel_stack_area,
            user_stack_map_area: user_stack_area,
            exit_code: None,
        };

        Self { tid: tid, process: Arc::downgrade(&process), inner: Arc::new(Mutex::new(task_inner)) }
    }

    pub fn copy(&self, new_process: Arc<ProcessControlBlock>) -> Self {
        let task_inner = self.inner.lock();
        let mut process_inner = new_process.inner.lock();
        
        let new_user_stack_area = if let Some(user_stack_map_area) = task_inner.user_stack_map_area.as_ref() {
            if user_stack_map_area.map_perm.contains(MapPermission::W) {
                let mut map_perm: MapPermission = user_stack_map_area.map_perm;
                map_perm.remove(MapPermission::W);
                user_stack_map_area.change_perm(map_perm, &process_inner.memory_set.page_table);
            }
            Some(user_stack_map_area.copy())
        } else {
            None
        };
        
        // kernel stack
        let kernel_stack_vstub = alloc_kernel_virt_frame(KERNEL_STACK_PAGE_SIZE + 2).unwrap();
        let kernel_stack_bottom_vpn = VirtPageNum(kernel_stack_vstub.base_vpn.0 + 1);
        let kernel_stack_top_vpn = VirtPageNum(kernel_stack_bottom_vpn.0 + KERNEL_STACK_PAGE_SIZE);
        let mut kernel_stack_area = MapArea::new(
            kernel_stack_bottom_vpn..kernel_stack_top_vpn, 
            MapPermission::R | MapPermission::W
        );
        let is_modified = kernel_stack_area.map_if_need(&mut process_inner.memory_set.page_table);
        assert!(is_modified);

        let mut intr_cx = task_inner.intr_cx;
        // fork 系统调用使用该方法
        // 子进程的 fork 函数返回值为0，所以 eax 设置为 0
        intr_cx.eax = 0;
        assert_ne!(intr_cx.eip, 0);
        let task_inner = TaskControlBlockInner {
            status: task_inner.status,
            intr_cx,
            task_cx: TaskContext::go_to_intr_return(kernel_stack_top_vpn.base_address(), intr_cx),
            kernel_stack_vstub,
            kernel_stack_top_address: kernel_stack_top_vpn.base_address(),
            user_stack_top_address: task_inner.user_stack_top_address,
            kernel_stack_map_area: kernel_stack_area,
            user_stack_map_area: new_user_stack_area,
            exit_code: task_inner.exit_code.clone(),
        };

        Self { 
            tid: self.tid, 
            process: Arc::downgrade(&new_process), 
            inner: Arc::new(Mutex::new(task_inner)) 
        }
    }

    pub fn reset(&self, entry_point: usize, page_table: &PageTable) {
        assert_ne!(entry_point, 0);
        let mut task_inner = self.inner.lock();

        if let Some(mut map_area) = task_inner.user_stack_map_area.take() {
            map_area.unmap(page_table);
        }

        let user_stack_top_address = USER_STACK_TOP_VIRT_ADDRESS - (USER_STACK_SIZE + MEMORY_PAGE_SIZE) * self.tid;
        let user_stack_bottom_address = user_stack_top_address - USER_STACK_SIZE;
        let user_stack_top_va = VirtAddr(user_stack_top_address);
        let user_stack_bottom_va = VirtAddr(user_stack_bottom_address);
        let user_stack_area = MapArea::new(
            VirtPageNum::from(user_stack_bottom_va)..VirtPageNum::from(user_stack_top_va),
            MapPermission::R | MapPermission::W | MapPermission::U
        );

        let intr_context = IntrContext::user_intr_context(VirtAddr(entry_point), VirtAddr(user_stack_top_address));
        assert_ne!(intr_context.eip, 0);
        task_inner.status = TaskStatus::Ready;
        task_inner.intr_cx = intr_context;
        task_inner.user_stack_top_address = Some(user_stack_top_va);
        task_inner.user_stack_map_area = Some(user_stack_area);
    }

    fn push_data_to_user_stack<T>(intr_cx: &mut IntrContext, data: T) {
        assert!(core::mem::size_of::<T>() < MEMORY_PAGE_SIZE);
        let mut user_stack_top = intr_cx.esp;
        // 参数
        user_stack_top = user_stack_top - core::mem::size_of::<T>();
        unsafe {
            let data_ptr = user_stack_top as *mut T;
            *data_ptr = data;
        }
        // "函数返回地址"，假装是其他函数调用
        user_stack_top = user_stack_top - core::mem::size_of::<usize>();
        unsafe {
            let data_ptr = user_stack_top as *mut usize;
            *data_ptr = 0;
        }
        intr_cx.esp = user_stack_top;
    }

    pub fn destroy(&self, process_inner: &mut ProcessControlBlockInner) {
        let mut task_inner = self.inner.lock();
        if let Some(mut map_area) = task_inner.user_stack_map_area.take() {
            map_area.unmap(&mut process_inner.memory_set.page_table)
        }
        task_inner.kernel_stack_map_area.unmap(&mut process_inner.memory_set.page_table);
        process_inner.tid_allocator.dealloc(self.tid);
    }
}

impl Drop for TaskControlBlock {
    fn drop(&mut self) {
        if let Some(process) = self.process.upgrade() {
            let mut process_inner = process.inner.lock();
            self.destroy(&mut process_inner);
        }
    }
}

pub type ThreadIdAllocator = IdAllocator<THREAD_ID_BITMAP_SIZE>;

pub fn create_thread_id_allocator() -> ThreadIdAllocator {
    IdAllocator::new(Bitmap::<THREAD_ID_BITMAP_SIZE>::new([0; THREAD_ID_BITMAP_SIZE]), 0, 0, THREAD_MAX_ID, 0)
}
