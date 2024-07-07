use core::option::Option::Some;
use core::mem::drop;
use alloc::{sync::Arc, task, vec::Vec};
use manager::{add_task, fetch_task};
use processor::{schedule, take_current_task};
use switch::__switch;

use crate::{config::{HIGH_ADDRESS_BASE, MEMORY_PAGE_SIZE}, intr::IntrContext, mm::{MapArea, MapPermission, MapType, MemorySet, PageTable, PhysAddr, VPNRange, VirtAddr, KERNEL_PDT_ADDRESS}, process::{ProcessControlBlock, ProcessControlBlockInner, TaskContext, TaskControlBlock, TaskControlBlockInner, TaskStatus}};

mod switch;
mod manager;
mod processor;

pub use processor::run_tasks;

pub fn suspend_current_and_run_next() {
    if let Some(task) = take_current_task() {
        let mut task_inner = task.inner.lock();
        let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
        task_inner.status = TaskStatus::Ready;
        drop(task_inner);
        add_task(task);
        schedule(task_cx_ptr);
    }
}

pub fn thread_0() {
    loop {
        debug!("thread_0");
    }
}

pub fn thread_1() {
    loop {
        debug!("thread_1");
    }
}

pub fn test() {
    let kernel_process = {
        let memory_set = MemorySet::new(PageTable::from(PhysAddr(KERNEL_PDT_ADDRESS).into()), Vec::new());
        let kernel_process_inner = ProcessControlBlockInner::new(memory_set);
        Arc::new(ProcessControlBlock::new(0, kernel_process_inner))
    };

    let kstack_top = VirtAddr(usize::MAX).virt_page_num_floor().base_address();

    debug!("thread_0 address {:#x}", thread_0 as usize);
    let kstack_top0 = kstack_top;
    let kstack_base0 = VirtAddr(kstack_top0.0 - MEMORY_PAGE_SIZE);
    let stack_range0 = kstack_base0.virt_page_num_floor()..kstack_top0.virt_page_num_floor();
    debug!("thread_0 stack [{:#x}, {:#x})", stack_range0.start.base_address().0, stack_range0.end.base_address().0);
    let stack_area0 = MapArea::new(stack_range0, MapType::Framed, MapPermission::R | MapPermission::W);
    {
        let mut process_inner = kernel_process.inner.lock();
        process_inner.memory_set.add(stack_area0);
    }
    let intr_context0 = IntrContext::kernel_intr_context(VirtAddr(thread_0 as usize + HIGH_ADDRESS_BASE));
    let kernel_task0_inner = TaskControlBlockInner::new(
        TaskStatus::Ready, 
        intr_context0, 
        TaskContext::go_to_intr_return(kstack_top0, intr_context0)
    );
    let kernel_task0 = Arc::new(TaskControlBlock::new(kernel_process.clone(), kernel_task0_inner));
    kernel_process.inner.lock().tasks.push(Some(kernel_task0.clone()));

    debug!("thread_1 address {:#x}", thread_1 as usize);
    let kstack_top1 = VirtAddr(kstack_top.0 - MEMORY_PAGE_SIZE * 2);
    let kstack_base1 = VirtAddr(kstack_top1.0 - MEMORY_PAGE_SIZE);
    let stack_range1 = kstack_base1.virt_page_num_floor()..kstack_top1.virt_page_num_floor();
    let stack_area1 = MapArea::new(stack_range1, MapType::Framed, MapPermission::R | MapPermission::W);
    kernel_process.inner.lock().memory_set.add(stack_area1);
    let intr_context1 = IntrContext::kernel_intr_context(VirtAddr(thread_1 as usize + HIGH_ADDRESS_BASE));
    let kernel_task1_inner = TaskControlBlockInner::new(
        TaskStatus::Ready, 
        intr_context1, 
        TaskContext::go_to_intr_return(kstack_top1, intr_context1)
    );
    let kernel_task1 = Arc::new(TaskControlBlock::new(kernel_process.clone(), kernel_task1_inner));
    kernel_process.inner.lock().tasks.push(Some(kernel_task1.clone()));

    add_task(kernel_task0);
    add_task(kernel_task1);
    debug!("test done");
}
