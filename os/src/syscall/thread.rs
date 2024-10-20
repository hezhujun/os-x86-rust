use alloc::sync::Arc;

use crate::schedule::add_task;
use crate::{process::TaskControlBlock, schedule::current_task};


/// 功能：当前进程创建一个新的线程
/// 参数：entry 表示线程的入口函数地址，arg 表示传给线程入口函数参数
/// 返回值：创建的线程的 TID
/// syscall ID: 1000
pub fn sys_thread_create(entry: usize, arg: usize) -> isize {
    let current_task = current_task().unwrap();
    let process = current_task.process.upgrade().unwrap();
    let new_task = TaskControlBlock::new(process.clone(), entry, false, Some(arg));
    let tid = new_task.tid;
    let new_task = Arc::new(new_task);
    process.add_task(new_task.clone());
    add_task(new_task);
    tid as isize
}

pub fn sys_gettid() -> isize {
    let current_task = current_task().unwrap();
    current_task.tid as isize
}

/// 功能：等待当前进程内的一个指定线程退出
/// 参数：tid 表示指定线程的 TID
/// 返回值：如果线程不存在，返回-1；如果线程还没退出，返回-2；其他情况下，返回结束线程的退出码
/// syscall ID: 1002
pub fn sys_waittid(tid: usize) -> isize {
    let current_task = current_task().unwrap();
    let process = current_task.process.upgrade().unwrap();
    let mut process_inner = process.inner.lock();
    // a thread cannot wait for itself
    if current_task.tid == tid {
        return -1;
    }
    let mut exit_code: Option<isize> = None;
    if let Some(wait_task) = process_inner.tasks[tid].as_ref() {
        if let Some(waited_exit_code) = wait_task.inner.lock().exit_code.as_ref() {
            exit_code = Some(*waited_exit_code);
        }
    } else {
        // waited thread does not exist
        return -1;
    }

    if let Some(exit_code) = exit_code {
        // dealloc the exited thread
        let task = process_inner.tasks[tid].take();
        drop(process_inner);
        exit_code
    } else {
        -2
    }
}
