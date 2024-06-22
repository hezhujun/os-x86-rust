use core::option::Option::*;
use core::result::Result::*;
use core::{char, str::Chars};
use core::fmt::{self, Write};

use crate::arch::x86::{ScreenAttrChar, ScreenCharAttr};

const SCREEN_MAX_COL: usize = 80;
const SCREEN_MAX_ROW: usize = 25;
const SCREEN_BUFFER_LEN: usize = SCREEN_MAX_ROW * SCREEN_MAX_COL;

static mut SCREEN_CURSOR_POSITION: usize = 0;

fn get_screen_buffer() -> &'static mut [ScreenAttrChar] {
    unsafe {
        core::slice::from_raw_parts_mut(0xB8000 as *mut ScreenAttrChar, SCREEN_BUFFER_LEN)
    }
}

fn get_screen_cursor_position() -> usize {
    unsafe {
        SCREEN_CURSOR_POSITION
    }
}

fn set_screen_cursor_position(value: usize) {
    unsafe {
        SCREEN_CURSOR_POSITION = value;
    }
}

fn screen_init() {
    get_screen_buffer().iter_mut().for_each(|item| *item = ScreenAttrChar::default());
}

lazy_static! {
    static ref SCREEN: Screen = {
        screen_init();
        Screen
    };
}
pub struct Screen;

impl Screen {
    fn check_cursor_position(&self) {
        if get_screen_cursor_position() >= SCREEN_BUFFER_LEN {
            self.scroll_up_screen_one_line();
            self.check_cursor_position()
        }
    }

    fn add_cursor_position(&self) {
        set_screen_cursor_position(get_screen_cursor_position() + 1);
        self.check_cursor_position();
    }

    fn subtract_cursor_position(&self, to: usize) {
        if to >= get_screen_cursor_position() {
            return;
        }
        for idx in to..get_screen_cursor_position() {
            self.set_char(' ', idx, ScreenCharAttr::empty());
        }
        set_screen_cursor_position(to);
    }

    fn line_break(&self) {
        if get_screen_cursor_position() % SCREEN_BUFFER_LEN == 0 {
            return;
        }
        let new_position = (get_screen_cursor_position() + SCREEN_MAX_COL - 1) / SCREEN_MAX_COL * SCREEN_MAX_COL;
        set_screen_cursor_position(new_position);
        self.check_cursor_position();
    }

    fn set_char(&self, c: char, idx: usize, attr: ScreenCharAttr) {
        if idx >= SCREEN_BUFFER_LEN {
            return;
        }
        get_screen_buffer()[idx] = ScreenAttrChar::new(c as u8, attr);
    }

    fn put_char(&self, c: char, attr: ScreenCharAttr) {
        get_screen_buffer()[get_screen_cursor_position()] = ScreenAttrChar::new(c as u8, attr);
        self.add_cursor_position();
    }

    fn handle_char(&self, c: char, iter: &mut Chars, attr: ScreenCharAttr) {
        match c {
            ' ' ..= '~' => self.put_char(c, attr),
            '\r' => {
                if let Some(next_ch) = iter.next() {
                    match next_ch {
                        '\n' => self.line_break(),
                        _ => {
                            let line_start = get_screen_cursor_position() / SCREEN_MAX_COL * SCREEN_MAX_COL;
                            self.subtract_cursor_position(line_start);
                        }
                    }
                }
            }
            '\n' => self.line_break(),
            _ => {},
        }
    }

    pub fn print(&self, text: &str, attr: ScreenCharAttr) {
        let mut iter = text.chars();
        while let Some(c) = iter.next() {
            self.handle_char(c, &mut iter, attr);
        }
    }

    pub fn scroll_up_screen_one_line(&self) {
        let len = SCREEN_MAX_COL * (SCREEN_MAX_ROW - 1);
        for idx in 0..len {
            get_screen_buffer()[idx] = get_screen_buffer()[idx + SCREEN_MAX_COL];
        }
        for idx in 0..SCREEN_MAX_COL {
            get_screen_buffer()[len + idx] = ScreenAttrChar::default();
        }
        set_screen_cursor_position(get_screen_cursor_position() - SCREEN_MAX_COL);
    }
}

struct ScreenStdout;

impl Write for ScreenStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        SCREEN.print(s, ScreenCharAttr::FOREGROUND_R | ScreenCharAttr::FOREGROUND_G | ScreenCharAttr::FOREGROUND_B);
        Ok(())
    }
}

pub fn screen_print(args: fmt::Arguments) {
    ScreenStdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! screen_print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::drivers::screen::screen_print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! screen_println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::drivers::screen::screen_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
