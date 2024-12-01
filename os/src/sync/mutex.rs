use core::marker::Send;
use core::marker::Sync;
use alloc::collections::VecDeque;
use alloc::sync::Arc;

use spin;

use crate::process::TaskControlBlock;
use crate::schedule::block_current_and_run_next;
use crate::schedule::current_task;
use crate::schedule::suspend_current_and_run_next;
use crate::schedule::wakeup_task;

pub trait Mutex: Send + Sync {
    fn lock(&self);
    fn unlock(&self);
}

pub struct MutexSpin {
    locked: spin::Mutex<bool>,
}

impl MutexSpin {
    pub fn new() -> Self {
        Self { locked: spin::Mutex::new(false) }
    }
}

impl Mutex for MutexSpin {
    fn lock(&self) {
        loop {
            let mut locked = self.locked.lock();
            if *locked {
                drop(locked);
                suspend_current_and_run_next();
                continue;
            } else {
                *locked = true;
                return;
            }
        }
    }

    fn unlock(&self) {
        let mut locked = self.locked.lock();
        assert!(*locked);
        *locked = false;
    }
}

pub struct MutexBlocking {
    inner: spin::Mutex<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    locked: bool,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    pub fn new() -> Self {
        Self { 
            inner: spin::Mutex::new(MutexBlockingInner { locked: false, wait_queue: VecDeque::new() })
        }
    }
}

impl Mutex for MutexBlocking {
    fn lock(&self) {
        let mut mutex_inner = self.inner.lock();
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            mutex_inner.locked = true;
        }
    }

    fn unlock(&self) {
        let mut mutex_inner = self.inner.lock();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            wakeup_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}