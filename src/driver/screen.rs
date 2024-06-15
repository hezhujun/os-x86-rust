use lazy_static::lazy_static;
use x86::{ScreenAttrChar, ScreenCharAttr};

const SCREEN_BUFFER_LEN: usize = 80 * 25;

fn get_screen_buffer() -> &'static mut [ScreenAttrChar] {
    unsafe {
        core::slice::from_raw_parts_mut(0xB8000 as *mut ScreenAttrChar, SCREEN_BUFFER_LEN)
    }
}

pub fn print_test() {
    let buffer = get_screen_buffer();
    for idx in 0..SCREEN_BUFFER_LEN {
        buffer[0] = ScreenAttrChar::default();
    }
    
    for (idx, c) in "Hello world".as_bytes().iter().enumerate() {
        let attrChar = ScreenAttrChar::new(*c, ScreenCharAttr::FOREGROUND_R);
        get_screen_buffer()[idx] = attrChar;
    }
}
