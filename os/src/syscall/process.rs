use crate::schedule::*;

pub fn sys_getpid() -> isize {
    if let Some(task) = current_task() {
        if let Some(process) = task.process.upgrade() {
            return process.get_pid().try_into().unwrap();
        }
    }
    -1
}

pub fn sys_exit(exit_code: isize) -> ! {
    exit_current_and_run_next(exit_code)
}
