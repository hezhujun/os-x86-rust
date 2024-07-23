use core::option::Option;
use core::option::Option::Some;
use core::option::Option::None;
use core::mem::drop;
use alloc::{sync::Arc, task, vec::Vec};
use manager::{add_task, fetch_task};
use processor::{schedule, take_current_task};
use switch::__switch;

use crate::config::*;
use crate::mm::*;
use crate::{config::MEMORY_PAGE_SIZE, intr::IntrContext, mm::{MapArea, MapPermission, MemorySet, PageTable, PhysAddr, VPNRange, VirtAddr}, process::{ProcessControlBlock, ProcessControlBlockInner, TaskContext, TaskControlBlock, TaskControlBlockInner, TaskStatus}};

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

pub fn init() {
    
}

static mut PROCESS_LIST: Option<[Arc<ProcessControlBlock>; 3]> = None;

pub fn test() {
    extern "C" {
        fn app_0_start();
        fn app_0_end();
        fn app_1_start();
        fn app_1_end();
        fn app_2_start();
        fn app_2_end();
    }

    let app_0_data = unsafe {
        core::slice::from_raw_parts(app_0_start as usize as *const u8, app_0_end as usize - app_0_start as usize)
    };
    let app_1_data = unsafe {
        core::slice::from_raw_parts(app_1_start as usize as *const u8, app_1_end as usize - app_1_start as usize)
    };
    let app_2_data = unsafe {
        core::slice::from_raw_parts(app_2_start as usize as *const u8, app_2_end as usize - app_2_start as usize)
    };

    let process0 = ProcessControlBlock::new(app_0_data);
    let task0 = {
        let inner = process0.inner.lock();
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };
    let process1 = ProcessControlBlock::new(app_1_data);
    let task1 = {
        let inner = process1.inner.lock();
        inner.tasks[1].as_ref().map(|task| task.clone()).unwrap()
    };
    let process2 = ProcessControlBlock::new(app_2_data);
    let task2 = {
        let inner = process2.inner.lock();
        inner.tasks[2].as_ref().map(|task| task.clone()).unwrap()
    };

    add_task(task0);
    add_task(task1);
    add_task(task2);

    unsafe {
        PROCESS_LIST = Some([process0, process1, process2]);
    }

    debug!("test done");
}
