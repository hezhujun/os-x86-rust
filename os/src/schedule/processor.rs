use core::arch::asm;

use alloc::sync::Arc;
use spin::Mutex;

use crate::config::*;
use crate::intr;
use crate::mm::PageTable;
use crate::process::ProcessControlBlock;
use crate::process::{TaskContext, TaskControlBlock, TaskStatus};
use crate::mm::update_tss;
use super::DATA_SELECTOR;
use super::{manager::fetch_task, switch::__switch};


pub struct Processor {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self { current: None, idle_task_cx: TaskContext::empty() }
    }

    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(|task| task.clone() )
    }

    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
}

lazy_static! {
    pub static ref PROCESSOR: Arc<Mutex<Processor>> = Arc::new(Mutex::new(Processor::new()));
}

pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.lock();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            let mut task_inner = task.inner.lock();
            let process = task.process.upgrade().unwrap();
            let process_inner = process.inner.lock();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.status = TaskStatus::Running;
            // 更新 tss
            {
                update_tss(DATA_SELECTOR as usize, task_inner.kernel_stack_top_address.0);
            }
            
            let pdt_ppn = process_inner.memory_set.page_table.pdt_ppn;
            // 切换页表
            unsafe {
                asm!("mov cr3, {}", in(reg) pdt_ppn.base_address().0);
            }
            assert_eq!(pdt_ppn, PageTable::pdt_ppn());

            assert_ne!(task_inner.intr_cx.eip, 0, "process {} intr_cx.eip == 0", process.get_pid());
            
            drop(process_inner);
            drop(task_inner);
            processor.current = Some(task);
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("no tasks available in run_tasks");
        }
    }
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.lock().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.lock().current()
}

pub fn current_process() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.lock().current().map(|task| {
        task.process.upgrade().unwrap()
    })
}

pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.lock();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr)
    }
}