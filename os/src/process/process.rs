use core::option::Option;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use crate::mm::alloc_kernel_virt_frame;
use crate::mm::memory_set;
use crate::mm::MemorySet;
use crate::config::*;
use crate::utils::*;
use super::task::*;
use super::task::create_thread_id_allocator;

pub struct ProcessControlBlockInner {
    pub memory_set: MemorySet,
    pub tid_allocator: ThreadIdAllocator,
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
}

impl ProcessControlBlockInner {
    pub fn new(memory_set: MemorySet) -> Self {
        Self { memory_set: memory_set, tid_allocator: create_thread_id_allocator(), tasks: Vec::new() }
    }
}

pub struct ProcessControlBlock {
    pub pid_stub: ProcessIdStub,
    pub inner: Arc<Mutex<ProcessControlBlockInner>>,
}

impl ProcessControlBlock {
    pub fn from_elf_file(elf_data: &[u8]) -> Arc<Self> {
        // 1. alloc pid
        let pid_stub = alloc_process_id().unwrap();
        // 2. alloc memory space
        let (memory_set, entry_point) = MemorySet::from_elf(elf_data);
        let inner = ProcessControlBlockInner::new(memory_set);
        let process = ProcessControlBlock { pid_stub, inner: Arc::new(Mutex::new(inner)) };
        let process = Arc::new(process);
        let task = TaskControlBlock::new(process.clone(), entry_point, false);
        process.add_task(Arc::new(task));
        process
    }

    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        let pid_stub = alloc_process_id().unwrap();
        let (memory_set, entry_point) = MemorySet::from_elf(elf_data);
        let inner = ProcessControlBlockInner::new(memory_set);
        let process = ProcessControlBlock { pid_stub, inner: Arc::new(Mutex::new(inner)) };
        let process = Arc::new(process);
        let task = TaskControlBlock::new(process.clone(), entry_point, false);
        process.add_task(Arc::new(task));
        process
    }

    pub fn new_kernel_process(entry_point: usize) -> Arc<Self> {
        let pid_stub = alloc_process_id().unwrap();
        let memory_set = MemorySet::new_kernel_memory_set();
        let inner = ProcessControlBlockInner::new(memory_set);
        let process = ProcessControlBlock { pid_stub, inner: Arc::new(Mutex::new(inner)) };
        let process = Arc::new(process);
        let task = TaskControlBlock::new(process.clone(), entry_point, true);
        process.add_task(Arc::new(task));
        process
    }

    pub fn get_pid(&self) -> usize {
        self.pid_stub.get_id()
    }

    pub fn add_task(&self, task: Arc<TaskControlBlock>) {
        let mut inner = self.inner.lock();
        let tid = task.tid;
        while tid >= inner.tasks.len() {
            inner.tasks.push(None);
        }
        inner.tasks[tid] = Some(task);
    }
}

lazy_static! {
    static ref PROCESS_ID_ALLOCATOR: Arc<Mutex<IdAllocator<PROCESS_ID_BITMAP_SIZE>>> = {
        Arc::new(Mutex::new(IdAllocator::new(Bitmap::<PROCESS_ID_BITMAP_SIZE>::new([0; PROCESS_ID_BITMAP_SIZE]), 0, 0, PROCESS_MAX_ID, 0)))
    };
}

pub struct ProcessIdStub {
    id_stub: IdStub<PROCESS_ID_BITMAP_SIZE>
}

impl ProcessIdStub {
    fn new(id: usize) -> Self {
        Self { id_stub: IdStub::new(id, PROCESS_ID_ALLOCATOR.clone()) }
    }

    fn get_id(&self) -> usize {
        self.id_stub.id
    }
}

fn alloc_process_id() -> Option<ProcessIdStub> {
    PROCESS_ID_ALLOCATOR.lock().alloc().map(|id| ProcessIdStub::new(id))
}
