use core::option::Option;
use core::option::Option::Some;
use core::option::Option::None;
use core::mem::drop;
use alloc::{sync::Arc, task, vec::Vec};
use manager::{add_task, fetch_task};
use processor::current_task;
use processor::{schedule, take_current_task};
use switch::__switch;

use crate::config::*;
use crate::intr;
use crate::mm::*;
use crate::{config::MEMORY_PAGE_SIZE, intr::IntrContext, mm::{MapArea, MapPermission, MemorySet, PageTable, PhysAddr, VPNRange, VirtAddr}, process::{ProcessControlBlock, ProcessControlBlockInner, TaskContext, TaskControlBlock, TaskControlBlockInner, TaskStatus}};

mod switch;
mod manager;
mod processor;

pub use processor::run_tasks;

pub fn suspend_current_and_run_next() {
    if let Some(task) = take_current_task() {
        let mut task_inner = task.task_inner.lock();
        let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
        task_inner.status = TaskStatus::Ready;
        drop(task_inner);
        add_task(task);
        schedule(task_cx_ptr);
    }
}


pub fn handle_page_fault() {
    if let Some(task) = current_task() {
        if let Some(process) = task.process.upgrade() {
            let mut process_inner = process.inner.lock();
            let memory_set = &process_inner.memory_set;
            let page_table = &memory_set.page_table;
            let mut is_mapped = false;
            if let Some(map_area) = memory_set.areas.first() {
                let start_vpn = map_area.vpn_range.start;
                is_mapped = page_table.is_vpn_present(start_vpn);
            }

            if !is_mapped {
                process_inner.map_all_areas_and_load_data();
            }

            return
        }
    }
    panic!("no process");
}

pub fn init() {
    
}

static mut PROCESS_LIST: Option<[Arc<ProcessControlBlock>; 5]> = None;

pub fn thread_0() {
    debug!("thread_0");
    loop {
        for i in 0..1000000 {
            debug!("thread_0 [{}]", i);
        }
    }
}

pub fn thread_1() {
    debug!("thread_1");
    loop {
        for i in 0..1000000 {
            debug!("thread_1 [{}]", i);
        }
    }
}

pub fn test() {
    extern "C" {
        fn app_0_start();
        fn app_0_end();
        fn app_1_start();
        fn app_1_end();
        fn app_2_start();
        fn app_2_end();
    }

    let app_0_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_0_start as usize as *const u8, app_0_end as usize - app_0_start as usize)
    };
    let app_1_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_1_start as usize as *const u8, app_1_end as usize - app_1_start as usize)
    };
    let app_2_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_2_start as usize as *const u8, app_2_end as usize - app_2_start as usize)
    };

    let process0 = ProcessControlBlock::from_elf_file(app_0_data);
    let task0 = {
        let mut inner = process0.inner.lock();
        inner.elf_data = Some(app_0_data);
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };
    let process1 = ProcessControlBlock::from_elf_file(app_1_data);
    let task1 = {
        let mut inner = process1.inner.lock();
        inner.elf_data = Some(app_1_data);
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };
    let process2 = ProcessControlBlock::from_elf_file(app_2_data);
    let task2 = {
        let mut inner = process2.inner.lock();
        inner.elf_data = Some(app_2_data);
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };

    debug!("thread_0 address {:#x}", thread_0 as usize);
    let process3 = ProcessControlBlock::new_kernel_process(thread_0 as usize);
    let task3 = {
        let inner = process3.inner.lock();
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };
    debug!("thread_1 address {:#x}", thread_1 as usize);
    let process4 = ProcessControlBlock::new_kernel_process(thread_1 as usize);
    let task4 = {
        let inner = process4.inner.lock();
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };

    add_task(task3);
    add_task(task4);
    add_task(task0);
    add_task(task1);
    add_task(task2);

    unsafe {
        PROCESS_LIST = Some([process0, process1, process2, process3, process4]);
    }

    debug!("test done");
}
