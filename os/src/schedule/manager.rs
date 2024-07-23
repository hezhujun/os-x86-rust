use core::option::Option;
use alloc::{collections::{BTreeMap, VecDeque}, sync::Arc};
use spin::Mutex;

use crate::process::{ProcessControlBlock, TaskControlBlock};


pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>
}

impl TaskManager {
    pub fn new() -> Self {
        Self { ready_queue: VecDeque::new() }
    }

    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }

    pub fn remove(&mut self, task: Arc<TaskControlBlock>) {
        if let Some((id, _)) = self
            .ready_queue
            .iter()
            .enumerate()
            .find(|(_, t)| { Arc::as_ptr(t) == Arc::as_ptr(&task) }) {
            self.ready_queue.remove(id);
        }
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Arc<Mutex<TaskManager>> = Arc::new(Mutex::new(TaskManager::new()));
    pub static ref PID2PCB: Arc<Mutex<BTreeMap<usize, Arc<ProcessControlBlock>>>> = Arc::new(Mutex::new(BTreeMap::new()));
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    let mut manager = TASK_MANAGER.lock();
    manager.add(task);
}

pub fn remove_task(task: Arc<TaskControlBlock>) {
    let mut manager = TASK_MANAGER.lock();
    manager.remove(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    let mut manager = TASK_MANAGER.lock();
    manager.fetch()
}

pub fn pid2process(pid: usize) -> Option<Arc<ProcessControlBlock>> {
    let map = PID2PCB.lock();
    map.get(&pid).map(Arc::clone)
}

pub fn insert_into_pid2process(pid: usize, process: Arc<ProcessControlBlock>) {
    PID2PCB.lock().insert(pid, process);
}

pub fn remove_from_pid2process(pid: usize) {
    let mut map = PID2PCB.lock();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2process!", pid);
    }
}
