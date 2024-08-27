use core::option::Option;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use crate::mm::alloc_kernel_virt_frame;
use crate::mm::memory_set;
use crate::mm::MapPermission;
use crate::mm::MemorySet;
use crate::config::*;
use crate::utils::*;
use super::task::*;
use super::task::create_thread_id_allocator;

pub struct ProcessControlBlockInner {
    pub memory_set: MemorySet,
    pub tid_allocator: ThreadIdAllocator,
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    pub exit_code: isize,
    pub is_zombie: bool,
    pub elf_data: Option<&'static [u8]>,
}

impl ProcessControlBlockInner {
    pub fn new(memory_set: MemorySet) -> Self {
        Self { memory_set: memory_set, tid_allocator: create_thread_id_allocator(), tasks: Vec::new(), exit_code: 0, is_zombie: false, elf_data: None }
    }

    /// 修复缺页错误
    /// 返回内容：是否有修复页表
    pub fn repair_page_fault(&mut self) -> bool {
        let mut is_modified = false;
        for area in &mut self.memory_set.areas {
            is_modified |= area.map_if_need(&mut self.memory_set.page_table);
        }

        if is_modified {
            // 刚创建页表，拷贝数据
            if let Some(elf_data) = self.elf_data.as_ref() {
                for area in &self.memory_set.areas {
                    if !area.map_perm.contains(MapPermission::W) {
                        area.change_perm(area.map_perm | MapPermission::W, &self.memory_set.page_table);
                    }
                }
                if let Some(phs) = self.memory_set.program_headers.as_ref() {
                    for ph in phs {
                        let src = &elf_data[ph.file_offset..(ph.file_offset + ph.file_size)];
                        let dst = unsafe { core::slice::from_raw_parts_mut(ph.virtual_addr as *mut u8, ph.mem_size) };
                        dst.copy_from_slice(src);
                    }
                }
                for area in &self.memory_set.areas {
                    if !area.map_perm.contains(MapPermission::W) {
                        area.change_perm(area.map_perm, &self.memory_set.page_table);
                    }
                }
            } else {
                assert!(false, "no elf data")
            }
        } else {
            // 已经创建过页表，判断进程是否是有过 fork 操作
            for area in &mut self.memory_set.areas {
                if area.map_perm.contains(MapPermission::W) {
                    is_modified |= area.copy_if_need(&mut self.memory_set.page_table);
                }
            }
        }

        assert!(is_modified);

        is_modified
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
        // 3. alloc task resource
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

    pub fn fork(&self) -> Arc<Self> {
        // alloc pid
        let pid_stub = alloc_process_id().unwrap();

        let process_inner = self.inner.lock();
        // copy memory space
        let memory_set = process_inner.memory_set.copy();
        let tid_allocator = process_inner.tid_allocator;
        let tasks: Vec<Option<Arc<TaskControlBlock>>> = Vec::new();
        let inner = ProcessControlBlockInner {
            memory_set,
            tid_allocator,
            tasks,
            exit_code: process_inner.exit_code,
            is_zombie: process_inner.is_zombie,
            elf_data: process_inner.elf_data,
        };
        let new_process = ProcessControlBlock { pid_stub, inner: Arc::new(Mutex::new(inner)) };
        let new_process = Arc::new(new_process);

        // copy task
        let mut tasks: Vec<Option<Arc<TaskControlBlock>>> = Vec::new();
        for task_option in &process_inner.tasks {
            if let Some(task) = task_option.as_ref() {
                tasks.push(Some(Arc::new(task.copy(new_process.clone()))));
            } else {
                tasks.push(None);
            }
        }
        let _new_process = new_process.clone();
        let mut new_process_inner = _new_process.inner.lock();
        new_process_inner.tasks = tasks;
        drop(new_process_inner);

        new_process
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let mut process_inner = self.inner.lock();
        assert!(process_inner.tasks.len() == 1);

        let entry_point = process_inner.memory_set.reset_from_elf(elf_data);
        
        if let Some(task_option) = process_inner.tasks.first() {
            if let Some(task) = task_option {
                task.reset(entry_point, &process_inner.memory_set.page_table);
            }
        }
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
