use core::option::Option;
use core::option::Option::Some;
use core::option::Option::None;
use core::mem::drop;
use alloc::{sync::Arc, task, vec::Vec};
pub use manager::*;
pub use processor::current_task;
pub use processor::current_process;
use processor::{schedule, take_current_task};
use spin::Mutex;
use switch::__switch;

use crate::config::*;
use crate::intr::*;
use crate::mm::*;
use crate::process::KERNEL_PROCESS;
use crate::{config::MEMORY_PAGE_SIZE, intr::IntrContext, mm::{MapArea, MapPermission, MemorySet, PageTable, PhysAddr, VPNRange, VirtAddr}, process::{ProcessControlBlock, ProcessControlBlockInner, TaskContext, TaskControlBlock, TaskControlBlockInner, TaskStatus}};
use crate::programs::PROGRAMS;

mod switch;
mod manager;
mod processor;

pub use processor::run_tasks;

pub fn suspend_current_and_run_next() {
    check_current_process_status();
    if let Some(task) = take_current_task() {
        let mut task_inner = task.inner.lock();
        let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
        task_inner.status = TaskStatus::Ready;
        drop(task_inner);
        add_task(task);
        schedule(task_cx_ptr);
    } else {
        assert!(false);
    }
}

pub fn block_current_and_run_next() {
    if let Some(task) = take_current_task() {
        let mut task_inner = task.inner.lock();
        let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
        task_inner.status = TaskStatus::Block;
        drop(task_inner);
        schedule(task_cx_ptr);
    } else {
        assert!(false)
    }
}

pub fn wakeup_task(task: Arc<TaskControlBlock>) {
    let mut task_inner = task.inner.lock();
    task_inner.status = TaskStatus::Ready;
    drop(task_inner);
    add_task(task);
}

pub fn exit_current_and_run_next(exit_code: isize) -> ! {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner.lock();
    let process = task.process.upgrade().unwrap();
    let tid = task.tid;
    task_inner.exit_code = Some(exit_code);
    // can deallocate user space resources earlier
    drop(task_inner);
    drop(task);

    let mut process_inner = process.inner.lock();

    if tid == 0 {
        let pid = process.get_pid();
        remove_from_pid2process(pid);
        process_inner.exit_code = Some(exit_code);

        {
            let mut initproc_inner = INITPROC_PROCESS.inner.lock();
            for child in process_inner.children.iter_mut() {
                let mut child_inner = child.inner.lock();
                child_inner.parent = Some(Arc::downgrade(&INITPROC_PROCESS.clone()));
                initproc_inner.children.push(child.clone());
            }
            process_inner.children.clear();
        }

        process_inner
            .tasks
            .iter()
            .for_each(|task_option| {
                let task_option = task_option.as_ref();
                if let Some(task) = task_option {
                    let task_inner = task.inner.lock();
                    if task_inner.status == TaskStatus::Ready {
                        remove_task(task.clone());
                    }
                }
            })
    }

    if process_inner.tasks.iter()
        .all(|task_option| { 
            task_option.as_ref().map_or(true, |task| {
                let task_inner = task.inner.lock();
                task_inner.exit_code.is_some()
            }) 
        }) {
        process_inner.is_zombie = true;
    }
    
    drop(process_inner);
    drop(process);
    let mut _unused = TaskContext::empty();
    schedule(&mut _unused as *mut _);
    panic!("unreachable after sys_exit!");
}

pub fn check_current_process_status() {
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let process_inner = process.inner.lock();

    if process_inner.exit_code.is_some() {
        drop(process_inner);
        drop(process);
        drop(task);
        exit_current_and_run_next(0);
    }
}

fn page_fault_intr_handler(intr_context: &mut IntrContext) {
    let intr = intr_context.intr;
    let error_code = intr_context.error_code;
    let eip = intr_context.eip;
    let cs = intr_context.cs;
    let esp = intr_context.esp;
    let ss = intr_context.ss;
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let pid = process.get_pid();
    assert_ne!(eip, 0, "page_fault_intr_handler cs {:#x} eip {:#x} ss {:#x} esp {:#x} error code {} {} pid {}", cs, eip, ss, esp, error_code, IrqErrorCode(error_code), pid);
    // debug!("intr #{}({:#x}) error code {} {} eip {:#x} cs {:#x} esp {:#x} ss {:#x} ebp {:#x}", intr, intr, error_code, IrqErrorCode(error_code), eip, cs, esp, ss, intr_context.ebp);
    let mut process_inner = process.inner.lock();
    let mut is_repaired = process_inner.repair_page_fault();
    
    let memory_set = &mut process_inner.memory_set;
    let page_table = &mut memory_set.page_table;
    let mut task_inner = task.inner.lock();
    is_repaired |= task_inner.repair_page_fault(page_table);
    if !is_repaired {
        // if no repair operation, something error, exit process
        assert!(false);
    }

    assert_ne!(intr_context.eip, 0, "page_fault_intr_handler end with intr_context.eip=0");
}

pub fn init() {
    {
        // 初始化 KERNEL_PROCESS，不确定这段代码是否会被优化掉
        // 构建 0 号进程
        let kernel_process = &KERNEL_PROCESS;
        assert_eq!(kernel_process.get_pid(), 0);
    }
    {
        // 构建 1 号进程
        let initproc = &INITPROC_PROCESS;
        let task = {
            let inner = initproc.inner.lock();
            inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
        };
        add_task(task);
        insert_into_pid2process(initproc.get_pid(), INITPROC_PROCESS.clone());
    }
    INTR_HANDLER_TABLE.lock()[0xe] = page_fault_intr_handler;
}

lazy_static! {
    static ref INITPROC_PROCESS: Arc<ProcessControlBlock> = {
        let programs = PROGRAMS.lock();
        let elf: &'static [u8] = programs.get("initproc").unwrap();
        let process = ProcessControlBlock::from_elf_file(elf);
        let mut inner = process.inner.lock();
        inner.elf_data = Some(elf);
        assert_eq!(process.get_pid(), 1);
        process.clone()
    };


    static ref PROCESS_LIST: Arc<Mutex<Vec<Arc<ProcessControlBlock>>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn thread_0() {
    debug!("thread_0");
    loop {
        for i in 0..1000000 {
            info!("thread_0 [{}]", i);
        }
    }
}

pub fn thread_1() {
    debug!("thread_1");
    loop {
        for i in 0..1000000 {
            info!("thread_1 [{}]", i);
        }
    }
}

pub fn do_nothing() {
    debug!("do_nothing");
    loop {
        
    }
}

pub fn test() {
    let programs = PROGRAMS.lock();
    let app_0_data: &'static [u8] = programs.get("hello_world").unwrap();
    let app_1_data: &'static [u8] = programs.get("hello_world_a").unwrap();
    let app_2_data: &'static [u8] = programs.get("hello_world_b").unwrap();

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

    info!("thread_0 address {:#x}", thread_0 as usize);
    let process3 = ProcessControlBlock::new_kernel_process(thread_0 as usize);
    let task3 = {
        let inner = process3.inner.lock();
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };
    info!("thread_1 address {:#x}", thread_1 as usize);
    let process4 = ProcessControlBlock::new_kernel_process(thread_1 as usize);
    let task4 = {
        let inner = process4.inner.lock();
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };
    info!("do_nothing address {:#x}", do_nothing as usize);
    let process5 = ProcessControlBlock::new_kernel_process(do_nothing as usize);
    let task5 = {
        let inner = process5.inner.lock();
        inner.tasks[0].as_ref().map(|task| task.clone()).unwrap()
    };

    add_task(task0);
    insert_into_pid2process(0, process0.clone());
    add_task(task1);
    insert_into_pid2process(1, process1.clone());
    add_task(task2);
    insert_into_pid2process(2, process2.clone());
    add_task(task3);
    insert_into_pid2process(3, process3.clone());
    add_task(task4);
    insert_into_pid2process(4, process4.clone());
    add_task(task5);
    insert_into_pid2process(5, process5.clone());

    let mut process_list = PROCESS_LIST.lock();
    process_list.push(process0);
    process_list.push(process1);
    process_list.push(process2);
    process_list.push(process3);
    process_list.push(process4);
    process_list.push(process5);

    info!("test done");
}
