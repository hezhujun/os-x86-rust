

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        1 => {
            let text = unsafe { 
                let v = core::slice::from_raw_parts(buf, len);
                core::str::from_utf8_unchecked(v)
            };
            crate::drivers::Screen.print(text);
            len as isize
        }
        _ => {
            panic!("Unsupported fd {} in sys_write!", fd);
        }
    }
}
