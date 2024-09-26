use crate::{drivers::keyboard::get_char, schedule::current_task, screen_print};

pub fn sys_read(fd: usize, buf: *mut u8, len: usize) -> isize {
    if buf.is_null() {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let process_inner = process.inner.lock();
    if fd >= process_inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = process_inner.fd_table[fd].as_ref() {
        let file = file.clone();
        // file.read 可能会阻塞并切换进程，提前释放 process_inner，避免卡死
        drop(process_inner);
        if file.readable() {
            let buf = unsafe {
                core::slice::from_raw_parts_mut(buf, len)
            };
            file.read(buf) as isize
        } else {
            -2
        }
    } else {
        -1
    }
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if buf.is_null() {
        return 0;
    }

    if len == 0 {
        return 0;
    }

    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let process_inner = process.inner.lock();
    if fd >= process_inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = process_inner.fd_table[fd].as_ref() {
        let file = file.clone();
        drop(process_inner);
        if file.writable() {
            let buf = unsafe {
                core::slice::from_raw_parts(buf, len)
            };
            file.write(buf) as isize
        } else {
            -2
        }
    } else {
        -1
    }
}
