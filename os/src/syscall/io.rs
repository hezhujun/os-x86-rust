use crate::drivers::keyboard::get_char;

pub fn sys_read(fd: usize, buf: *mut u8, len: usize) -> isize {
    match fd {
        0 => {
            if buf.is_null() {
                return -1;
            }
            let buf = unsafe {
                core::slice::from_raw_parts_mut(buf, len)
            };
            let mut index = 0;
            while index < len {
                if let Some(ch) = get_char() {
                    buf[index] = ch;
                } else {
                    break;
                }
                index += 1;
            }
            index as isize
        }
        _ => {
            panic!("Unsupported fd {} in sys_write!", fd);
        }
    }
}

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
