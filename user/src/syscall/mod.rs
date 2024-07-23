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

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: isize) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}
