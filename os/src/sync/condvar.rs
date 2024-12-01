use alloc::sync::Arc;
use alloc::collections::VecDeque;
use spin::Mutex;

use crate::process::*;
use crate::schedule::*;
use crate::sync::Mutex as MyMutex;


pub struct Condvar {
    pub inner: Arc<Mutex<CondvarInner>>,
}

pub struct CondvarInner {
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Condvar {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CondvarInner {
                wait_queue: VecDeque::new(),
            })),
        }
    }

    pub fn signal(&self) {
        let mut inner = self.inner.lock();
        if let Some(task) = inner.wait_queue.pop_front() {
            wakeup_task(task);
        }
    }

    pub fn wait_with_mutex(&self, mutex: Arc<dyn MyMutex>) {
        let tid = current_task().unwrap().tid;
        mutex.unlock();
        let mut inner = self.inner.lock();
        inner.wait_queue.push_back(current_task().unwrap());
        drop(inner);
        block_current_and_run_next();
        mutex.lock();
    }
}
