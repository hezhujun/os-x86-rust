use alloc::sync::{Arc, Weak};
use spin::Mutex;

use crate::intr::IntrContext;
use super::{context::TaskContext, process::ProcessControlBlock};

pub enum TaskStatus {
    Ready, Running, Block
}

pub struct TaskControlBlockInner {
    pub status: TaskStatus,
    pub intr_cx: IntrContext,
    pub task_cx: TaskContext,
}

impl TaskControlBlockInner {
    pub fn new(status: TaskStatus, intr_cx: IntrContext, task_cx: TaskContext) -> Self {
        Self { status, intr_cx, task_cx }
    }
}

pub struct TaskControlBlock {
    pub process: Weak<ProcessControlBlock>,
    pub inner: Arc<Mutex<TaskControlBlockInner>>,
}

impl TaskControlBlock {
    pub fn new(process: Arc<ProcessControlBlock>, inner: TaskControlBlockInner) -> Self {
        Self { process: Arc::downgrade(&process.clone()), inner: Arc::new(Mutex::new(inner)) }
    }
}
