use core::option::Option;
use alloc::sync::Arc;
use alloc::task;
use alloc::vec::Vec;
use alloc::sync::Weak;
use spin::Mutex;
use alloc::vec;

use crate::arch::x86::PteFlags;
use crate::config::*;
use crate::mm::*;
use crate::sync::*;
use crate::utils::*;
use super::task::*;
use super::task::create_thread_id_allocator;
use crate::fs::*;
use crate::fs::stdio::*;
use crate::sync;

pub struct ProcessControlBlockInner {
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub memory_set: MemorySet,
    pub tid_allocator: ThreadIdAllocator,
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    pub exit_code: Option<isize>,
    pub is_zombie: bool,
    pub fd_table: Vec<Option<Arc<dyn File>>>,
    pub mutex_list: Vec<Option<Arc<dyn sync::Mutex>>>,
    pub semaphore_list: Vec<Option<Arc<sync::Semaphore>>>,
    pub condvar_list: Vec<Option<Arc<sync::Condvar>>>,
    pub elf_data: Option<&'static [u8]>,
}

impl ProcessControlBlockInner {
    pub fn new(memory_set: MemorySet) -> Self {
        Self { 
            parent: None, 
            children: Vec::new(), 
            memory_set: memory_set, 
            tid_allocator: create_thread_id_allocator(), 
            tasks: Vec::new(), 
            exit_code: None, 
            is_zombie: false, 
            fd_table: vec![
                // 0 -> stdin
                Some(Arc::new(Stdin)),
                // 1 -> stdout
                Some(Arc::new(Stdout)),
                // 2 -> stderr
                Some(Arc::new(Stdout)),
            ],
            mutex_list: Vec::new(),
            semaphore_list: Vec::new(),
            condvar_list: Vec::new(),
            elf_data: None,
        }
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
                        if ph.file_size == ph.mem_size {
                            dst.copy_from_slice(src);
                        } else {
                            dst.iter_mut().for_each(|b| *b = 0);
                        }
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

impl Drop for ProcessControlBlockInner {
    fn drop(&mut self) {
        let mut tasks = Vec::new();
        for task_option in self.tasks.iter_mut() {
            if let Some(task) = task_option.take() {
                tasks.push(task);
            }
        }
        for task in tasks.into_iter() {
            task.destroy(self);
        }
        let page_table = &self.memory_set.page_table;
        for area in &mut self.memory_set.areas {
            area.unmap(page_table);
        }
        self.memory_set.areas.clear();
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
        let task = TaskControlBlock::new::<()>(process.clone(), entry_point, false, None);
        process.add_task(Arc::new(task));
        process
    }

    pub fn new_kernel_process(entry_point: usize) -> Arc<Self> {
        let pid_stub = alloc_process_id().unwrap();
        let memory_set = MemorySet::new_kernel_memory_set();
        let inner = ProcessControlBlockInner::new(memory_set);
        let process = ProcessControlBlock { pid_stub, inner: Arc::new(Mutex::new(inner)) };
        let process = Arc::new(process);
        let task = TaskControlBlock::new::<()>(process.clone(), entry_point, true, None);
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

        let mut process_inner = self.inner.lock();
        assert!(process_inner.tasks.len() == 1);
        // copy memory space
        let memory_set = process_inner.memory_set.copy();
        let tid_allocator = process_inner.tid_allocator;
        let tasks: Vec<Option<Arc<TaskControlBlock>>> = Vec::new();
        // copy fd table
        let mut new_fd_table: Vec<Option<Arc<dyn File>>> = Vec::new();
        for fd in process_inner.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        let mut mutex_list: Vec<Option<Arc<dyn sync::Mutex>>> = Vec::new();
        for mutex_option in process_inner.mutex_list.iter() {
            if let Some(mutex) = mutex_option {
                mutex_list.push(Some(mutex.clone()));
            } else {
                mutex_list.push(None);
            }
        }
        let mut semaphore_list: Vec<Option<Arc<Semaphore>>> = Vec::new();
        for semaphore_option in process_inner.semaphore_list.iter() {
            if let Some(semaphore) = semaphore_option {
                semaphore_list.push(Some(semaphore.clone()));
            } else {
                semaphore_list.push(None);
            }
        }
        let mut condvar_list: Vec<Option<Arc<Condvar>>> = Vec::new();
        for condvar_option in process_inner.condvar_list.iter() {
            if let Some(condvar) = condvar_option {
                condvar_list.push(Some(condvar.clone()));
            } else {
                condvar_list.push(None);
            }
        }
        let inner = ProcessControlBlockInner {
            parent: None,
            children: Vec::new(),
            memory_set,
            tid_allocator,
            tasks,
            exit_code: process_inner.exit_code.clone(),
            is_zombie: process_inner.is_zombie,
            fd_table: new_fd_table,
            mutex_list: mutex_list,
            semaphore_list: semaphore_list,
            condvar_list: condvar_list,
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
        
        process_inner.children.push(new_process.clone());

        new_process
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let mut process_inner = self.inner.lock();
        assert!(process_inner.tasks.len() == 1);

        process_inner.elf_data = None;
        let entry_point = process_inner.memory_set.reset_from_elf(elf_data);
        
        if let Some(task_option) = process_inner.tasks.first() {
            if let Some(task) = task_option {
                task.reset(entry_point, &process_inner.memory_set.page_table);
            }
        }
    }
}

impl Drop for ProcessControlBlock {
    fn drop(&mut self) {
        let pid = self.get_pid();
        // debug!("drop process pid {}", pid);
    }
}

pub fn fork(process: Arc<ProcessControlBlock>) -> Arc<ProcessControlBlock> {
    let new_process = process.fork();
    {
        let mut new_process_inner = new_process.inner.lock();
        new_process_inner.parent = Some(Arc::downgrade(&process));
    }
    new_process
}

lazy_static! {
    static ref PROCESS_ID_ALLOCATOR: Arc<Mutex<IdAllocator<PROCESS_ID_BITMAP_SIZE>>> = {
        Arc::new(Mutex::new(IdAllocator::new(Bitmap::<PROCESS_ID_BITMAP_SIZE>::new([0; PROCESS_ID_BITMAP_SIZE]), 0, 0, PROCESS_MAX_ID, 0)))
    };

    pub static ref KERNEL_PROCESS: Arc<ProcessControlBlock> = {
        let pdt_pa = PhysAddr(KERNEL_PDT_PHYS_ADDRESS);
        let pdt_ppn = pdt_pa.phys_page_num_floor();
        let pdt_pstub = PhysFrameStub { base_ppn: pdt_ppn, len: 1 };
        let pdt_vstub = alloc_kernel_virt_frame(1).unwrap();
        let pdt_vpn = pdt_vstub.base_vpn;
        PageTable::static_map(pdt_vpn, pdt_ppn, PteFlags::P | PteFlags::RW);
        let page_table = PageTable::from_exists(pdt_ppn, pdt_vpn);
        // 内核的 user_stack_base 没有作用
        let memory_set = MemorySet::new(
            pdt_pstub,
            pdt_vstub,
            page_table,
            0
        );
        let process_inner = ProcessControlBlockInner {
            parent: None,
            children: Vec::new(),
            memory_set,
            tid_allocator: create_thread_id_allocator(),
            tasks: Vec::new(),
            exit_code: None,
            is_zombie: false,
            fd_table: vec![],
            mutex_list: Vec::new(),
            semaphore_list: Vec::new(),
            condvar_list: Vec::new(),
            elf_data: None,
        };

        let pid_stub = alloc_process_id().unwrap();
        let process = ProcessControlBlock {
            pid_stub,
            inner: Arc::new(Mutex::new(process_inner)),
        };
        Arc::new(process)
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
