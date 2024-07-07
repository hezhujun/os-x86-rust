use core::option::Option;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use crate::mm::MemorySet;

use super::task::TaskControlBlock;


pub struct ProcessControlBlockInner {
    pub memory_set: MemorySet,
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
}

impl ProcessControlBlockInner {
    pub fn new(memory_set: MemorySet) -> Self {
        Self { memory_set: memory_set, tasks: Vec::new() }
    }
}

pub struct ProcessControlBlock {
    pub pid: usize,
    pub inner: Arc<Mutex<ProcessControlBlockInner>>,
}

impl ProcessControlBlock {
    pub fn new(pid: usize, inner: ProcessControlBlockInner) -> Self {
        Self { pid, inner: Arc::new(Mutex::new(inner)) }
    }
}
