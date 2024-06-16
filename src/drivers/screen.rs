use lazy_static::lazy_static;
use crate::arch::x86::{ScreenAttrChar, ScreenCharAttr};

const SCREEN_MAX_COL: usize = 80;
const SCREEN_MAX_ROW: usize = 25;
const SCREEN_BUFFER_LEN: usize = SCREEN_MAX_ROW * SCREEN_MAX_COL;

static mut CURRENT_ROW: usize = 0;
static mut CURRENT_COL: usize = 0;

fn get_screen_buffer() -> &'static mut [ScreenAttrChar] {
    unsafe {
        core::slice::from_raw_parts_mut(0xB8000 as *mut ScreenAttrChar, SCREEN_BUFFER_LEN)
    }
}

fn current_position() -> usize {
    unsafe {
        if CURRENT_ROW == 0 {
            CURRENT_COL
        } else {
            CURRENT_ROW * SCREEN_MAX_COL + CURRENT_COL
        }
    }
}

pub fn screen_init() {
    get_screen_buffer().iter_mut().for_each(|item| *item = ScreenAttrChar::default());
}

pub fn print(text: &str) {
    for (idx, c) in text.as_bytes().iter().enumerate() {
        let screen_buffer = get_screen_buffer();
        let attr_char = ScreenAttrChar::new(*c, ScreenCharAttr::FOREGROUND_R);
        screen_buffer[current_position()] = attr_char;
        unsafe {
            CURRENT_COL += 1;
            if CURRENT_COL >= SCREEN_MAX_COL {
                CURRENT_ROW += 1;
                CURRENT_COL = 0;
            }
            if CURRENT_ROW >= SCREEN_MAX_ROW {
                CURRENT_ROW = SCREEN_MAX_ROW - 1;
                scroll_up_screen_one_line(screen_buffer);
            }
        }
    }
}

pub fn println(text: &str) {
    print(text);
    unsafe {
        if CURRENT_COL == 0 {
            return
        }
        CURRENT_COL = 0;
        CURRENT_ROW += 1;
        if CURRENT_ROW >= SCREEN_MAX_ROW {
            CURRENT_ROW = SCREEN_MAX_ROW - 1;
            scroll_up_screen_one_line(get_screen_buffer());
        }
    }
}

fn scroll_up_screen_one_line(screen_buffer: &mut [ScreenAttrChar]) {
    let len = SCREEN_MAX_COL * (SCREEN_MAX_ROW - 1);
    for idx in 0..len {
        screen_buffer[idx] = screen_buffer[idx + SCREEN_MAX_COL];
    }
    for idx in 0..SCREEN_MAX_COL {
        screen_buffer[len + idx] = ScreenAttrChar::default();
    }
}
