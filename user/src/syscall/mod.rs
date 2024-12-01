mod define;

use define::*;

use core::arch::asm;
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "int 0x80",
            inlateout("eax") id => ret,
            in("ebx") args[0],
            in("ecx") args[1],
            in("edx") args[2],
        );
    }
    ret
}

pub fn sys_read(fd: usize, buffer: &mut[u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: isize) -> ! {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0]);
    panic!("unreachable after sys_exit!");
}

pub fn sys_sleep(sleep_ms: usize) -> isize {
    syscall(SYSCALL_SLEEP, [sleep_ms, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0,0 ])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

/// 功能：当前进程 fork 出来一个子进程。
/// 返回值：对于子进程返回 0，对于当前进程则返回子进程的 PID 。
/// syscall ID：220
pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

/// 功能：将当前进程的地址空间清空并加载一个特定的可执行文件，返回用户态后开始它的执行。
/// 参数：path 给出了要加载的可执行文件的名字；
/// 返回值：如果出错的话（如找不到名字相符的可执行文件）则返回 -1，否则不应该返回。
/// syscall ID：221
pub fn sys_exec(path: &str, args: &[*const u8]) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, args.as_ptr() as usize, 0])
}

/// 功能：当前进程等待一个子进程变为僵尸进程，回收其全部资源并收集其返回值。
/// 参数：pid 表示要等待的子进程的进程 ID，如果为 -1 的话表示等待任意一个子进程；
/// exit_code 表示保存子进程返回值的地址，如果这个地址为 0 的话表示不必保存。
/// 返回值：如果要等待的子进程不存在则返回 -1；否则如果要等待的子进程均未结束则返回 -2；
/// 否则返回结束的子进程的进程 ID。
/// syscall ID：260
pub fn sys_waitpid(pid: isize, exit_code: *mut isize) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}


/// 功能：当前进程创建一个新的线程
/// 参数：entry 表示线程的入口函数地址，arg 表示传给线程入口函数参数
/// 返回值：创建的线程的 TID
/// syscall ID: 1000
pub fn sys_thread_create(entry: usize, arg: usize) -> isize {
    syscall(SYSCALL_THREAD_CREATE, [entry, arg, 0])
}

pub fn sys_gettid() -> isize {
    syscall(SYSCALL_GETTID, [0, 0, 0])
}

/// 功能：等待当前进程内的一个指定线程退出
/// 参数：tid 表示指定线程的 TID
/// 返回值：如果线程不存在，返回-1；如果线程还没退出，返回-2；其他情况下，返回结束线程的退出码
/// syscall ID: 1002
pub fn sys_waittid(tid: usize) -> isize {
    syscall(SYSCALL_WAITTID, [tid, 0, 0])
}

/// 功能：为当前进程新增一把互斥锁。
/// 参数： blocking 为 true 表示互斥锁基于阻塞机制实现，
/// 否则表示互斥锁基于类似 yield 的方法实现。
/// 返回值：假设该操作必定成功，返回创建的锁的 ID 。
/// syscall ID: 1010
pub fn sys_mutex_create(blocking: bool) -> isize {
    syscall(SYSCALL_MUTEX_CREATE, [if blocking { 1 } else { 0 }, 0, 0])
}

/// 功能：当前线程尝试获取所属进程的一把互斥锁。
/// 参数： mutex_id 表示要获取的锁的 ID 。
/// 返回值： 0
/// syscall ID: 1011
pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    syscall(SYSCALL_MUTEX_LOCK, [mutex_id, 0, 0])
}

/// 功能：当前线程释放所属进程的一把互斥锁。
/// 参数： mutex_id 表示要释放的锁的 ID 。
/// 返回值： 0
/// syscall ID: 1012
pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    syscall(SYSCALL_MUTEX_UNLOCK, [mutex_id, 0, 0])
}


/// 功能：为当前进程新增一个信号量。
/// 参数：res_count 表示该信号量的初始资源可用数量，即 N ，为一个非负整数。
/// 返回值：假定该操作必定成功，返回创建的信号量的 ID 。
/// syscall ID : 1020
pub fn sys_semaphore_create(res_count: usize) -> isize {
    syscall(SYSCALL_SEMAPHORE_CREATE, [res_count, 0, 0])
}

/// 功能：对当前进程内的指定信号量进行 V 操作。
/// 参数：sem_id 表示要进行 V 操作的信号量的 ID 。
/// 返回值：假定该操作必定成功，返回 0 。
pub fn sys_semaphore_up(sem_id: usize) -> isize {
    syscall(SYSCALL_SEMAPHORE_UP, [sem_id, 0, 0])
}

/// 功能：对当前进程内的指定信号量进行 P 操作。
/// 参数：sem_id 表示要进行 P 操作的信号量的 ID 。
/// 返回值：假定该操作必定成功，返回 0 。
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    syscall(SYSCALL_SEMAPHORE_DOWN, [sem_id, 0, 0])
}


/// 功能：为当前进程新增一个条件变量。
/// 返回值：假定该操作必定成功，返回创建的条件变量的 ID 。
/// syscall ID : 1030
pub fn sys_condvar_create() -> isize {
    syscall(SYSCALL_CONDVAR_CREATE, [0, 0, 0])
}

/// 功能：对当前进程的指定条件变量进行 signal 操作，即
/// 唤醒一个在该条件变量上阻塞的线程（如果存在）。
/// 参数：condvar_id 表示要操作的条件变量的 ID 。
/// 返回值：假定该操作必定成功，返回 0 。
/// syscall ID : 1031
pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    syscall(SYSCALL_CONDVAR_SIGNAL, [condvar_id, 0, 0])
}

/// 功能：对当前进程的指定条件变量进行 wait 操作，分为多个阶段：
/// 1. 释放当前线程持有的一把互斥锁；
/// 2. 阻塞当前线程并将其加入指定条件变量的阻塞队列；
/// 3. 直到当前线程被其他线程通过 signal 操作唤醒；
/// 4. 重新获取当前线程之前持有的锁。
/// 参数：mutex_id 表示当前线程持有的互斥锁的 ID ，而
/// condvar_id 表示要操作的条件变量的 ID 。
/// 返回值：假定该操作必定成功，返回 0 。
/// syscall ID : 1032
pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    syscall(SYSCALL_CONDVAR_WAIT, [condvar_id, mutex_id, 0])
}
