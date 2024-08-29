use alloc::sync::Arc;

use crate::intr::IntrContext;
use crate::{process::fork, schedule::*};
use crate::process;
use crate::programs::PROGRAMS;

pub fn sys_exit(exit_code: isize) -> ! {
    exit_current_and_run_next(exit_code)
}


pub fn sys_sleep() {

}


pub fn sys_yield() {
    suspend_current_and_run_next();
}

pub fn sys_getpid() -> isize {
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    process.get_pid().try_into().unwrap()
}

/// 功能：当前进程 fork 出来一个子进程。
/// 返回值：对于子进程返回 0，对于当前进程则返回子进程的 PID 。
/// syscall ID：220
pub fn sys_fork() -> isize {
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let new_process = fork(process);
    {
        let new_process_inner = new_process.inner.lock();
        if let Some(task_option) = new_process_inner.tasks.first() {
            if let Some(task) = task_option {
                add_task(task.clone());
            }
        }
    }
    new_process.get_pid() as isize
}

/// 功能：将当前进程的地址空间清空并加载一个特定的可执行文件，返回用户态后开始它的执行。
/// 参数：path 给出了要加载的可执行文件的名字；
/// 返回值：如果出错的话（如找不到名字相符的可执行文件）则返回 -1。
/// syscall ID：221
pub fn sys_exec(path: &str, args: &[*const u8], intr_cx: &mut IntrContext) -> isize {
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let programs = PROGRAMS.lock();
    if let Some(elf_data) = programs.get(path) {
        process.exec(elf_data);
        let task_inner = task.inner.lock();
        *intr_cx = task_inner.intr_cx;
        0
    } else {
        -1
    }
}

/// 功能：当前进程等待一个子进程变为僵尸进程，回收其全部资源并收集其返回值。
/// 参数：pid 表示要等待的子进程的进程 ID，如果为 -1 的话表示等待任意一个子进程；
/// exit_code 表示保存子进程返回值的地址，如果这个地址为 0 的话表示不必保存。
/// 返回值：如果要等待的子进程不存在则返回 -1；否则如果要等待的子进程均未结束则返回 -2；
/// 否则返回结束的子进程的进程 ID。
/// syscall ID：260
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let mut process_inner = process.inner.lock();
    let info = process_inner.children.iter().enumerate().map(|(index, child)| {
        let child_inner = child.inner.lock();
        (index, child.get_pid(), child_inner.is_zombie)
    }).find(|(index, child_pid, is_zombie)| {
        if pid == -1 {
            *is_zombie
        } else {
            *child_pid == pid as usize
        }
    });
    
    if let Some((child_index, child_pid, is_zombie)) = info {
        if pid != -1 && !is_zombie {
            // 进程未结束
            return -2
        }
        let child = process_inner.children.remove(child_index);
        let child_inner = child.inner.lock();
        assert!(child_inner.is_zombie);
        for child_of_child in child_inner.children.iter() {
            let mut child_of_child_inner = child_of_child.inner.lock();
            child_of_child_inner.parent = Some(Arc::downgrade(&process));
            process_inner.children.push(child_of_child.clone());
        }
        child_pid.try_into().unwrap()
    } else {
        if pid == -1 {
            -2
        } else {
            // can not find process
            -1
        }
    }
}
