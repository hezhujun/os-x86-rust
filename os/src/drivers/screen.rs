use core::any::Any;
use core::arch::asm;
use core::option::Option::*;
use core::result::Result::*;
use core::{char, str::Chars};
use core::fmt::{self, Write};

use alloc::sync::Arc;
use spin::Mutex;

use crate::arch::x86::{ByteWriteByteReadPort, Eflags, ScreenAttrChar, ScreenCharAttr};

const SCREEN_MAX_COL: usize = 80;
const SCREEN_MAX_ROW: usize = 25;
const SCREEN_BUFFER_LEN: usize = SCREEN_MAX_ROW * SCREEN_MAX_COL;

static mut DEFAULT_SCREEN_DRIVER: Option<DefaultScreenDriver> = Some(DefaultScreenDriver { cursor: 0 });
static mut USE_SCREEN_DRIVER: bool = false;

pub fn init() {
    unsafe {
        USE_SCREEN_DRIVER = true;
    }
}

fn get_screen_buffer() -> &'static mut [ScreenAttrChar] {
    unsafe {
        core::slice::from_raw_parts_mut(0xC00B8000 as *mut ScreenAttrChar, SCREEN_BUFFER_LEN)
    }
}

fn screen_init() {
    get_screen_buffer().iter_mut().for_each(|item| *item = ScreenAttrChar::default());
}

lazy_static! {
    static ref SCREEN: Screen = {
        unsafe {
            if let Some(screen) = DEFAULT_SCREEN_DRIVER.as_mut() {
                screen.init();
            }
        }
        Screen
    };

    static ref SCREEN_DRIVER: Arc<Mutex<dyn ScreenDriver>> = unsafe {
        let screen = if let Some(screen) = DEFAULT_SCREEN_DRIVER.take() {
            screen
        } else {
            DefaultScreenDriver::new()
        };
        Arc::new(Mutex::new(screen))
    };
}
pub struct Screen;

impl Screen {
    pub fn get_cursor(&self) -> usize {
        unsafe {
            if let Some(screen) = DEFAULT_SCREEN_DRIVER.as_ref() {
                screen.get_cursor()
            } else {
                0
            }
        }
    }

    pub fn set_cursor(&self, index: usize) {
        unsafe {
            if let Some(screen) = DEFAULT_SCREEN_DRIVER.as_mut() {
                screen.set_cursor(index);
            }
        }
    }

    pub fn print_char(&self, ch: char, attr: ScreenCharAttr) {
        unsafe {
            if let Some(screen) = DEFAULT_SCREEN_DRIVER.as_mut() {
                screen.print_char(ch, attr);
            }
        }
    }

    pub fn print_str(&self, text: &str, attr: ScreenCharAttr) {
        unsafe {
            if let Some(screen) = DEFAULT_SCREEN_DRIVER.as_mut() {
                screen.print_str(text, attr);
            }
        }
    }

    pub fn get_char(&self, index: usize) -> Option<ScreenAttrChar> {
        unsafe {
            if let Some(screen) = DEFAULT_SCREEN_DRIVER.as_mut() {
                screen.get_char(index)
            } else {
                None
            }
        }
    }
}

struct ScreenStdout;

impl Write for ScreenStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if unsafe { USE_SCREEN_DRIVER } {
            SCREEN_DRIVER.lock().print_str(s, ScreenCharAttr::default());
        } else {
            SCREEN.print_str(s, ScreenCharAttr::default());
        }
        Ok(())
    }
}

pub fn screen_print(args: fmt::Arguments) {
    let old_eflags = Eflags::read();
    if old_eflags.contains(Eflags::IF) {
        unsafe {
            asm!("cli");
        }
    }
    ScreenStdout.write_fmt(args).unwrap();
    if old_eflags.contains(Eflags::IF) {
        unsafe {
            asm!("sti");
        }
    }
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

pub trait ScreenDriver: Send + Sync + Any  {
    fn get_cursor(&self) -> usize;
    fn set_cursor(&mut self, index: usize);
    fn print_char(&mut self, ch: char, attr: ScreenCharAttr);
    fn print_str(&mut self, text: &str, attr: ScreenCharAttr);
    fn get_char(&self, index: usize) -> Option<ScreenAttrChar>;
}

pub struct DefaultScreenDriver {
    cursor: usize
}

impl DefaultScreenDriver {
    pub fn new() -> Self {
        let mut screen = Self { cursor: 0 };
        screen.init();
        screen
    }

    pub fn from(other: &Self) -> Self {
        Self { cursor: other.cursor }
    }

    fn init(&mut self) {
        screen_init();
        self.set_cursor(0);
    }
}

impl DefaultScreenDriver {
    fn check_cursor_position(&mut self) {
        if self.cursor >= SCREEN_BUFFER_LEN {
            self.scroll_up_screen_one_line();
            self.check_cursor_position();
        }
    }

    fn add_cursor_position(&mut self) {
        self.cursor += 1;
        self.check_cursor_position();
    }

    fn subtract_cursor_position(&mut self, to: usize) {
        if to >= self.cursor {
            return;
        }
        for idx in to..self.cursor {
            self.set_char(' ', idx, ScreenCharAttr::default());
        }
        self.cursor = to;
    }

    fn set_char(&self, c: char, idx: usize, attr: ScreenCharAttr) {
        if idx >= SCREEN_BUFFER_LEN {
            return;
        }
        get_screen_buffer()[idx] = ScreenAttrChar::new(c as u8, attr);
    }

    fn line_break(&mut self) {
        let new_position = (self.cursor + SCREEN_MAX_COL - 1) / SCREEN_MAX_COL * SCREEN_MAX_COL;
        self.cursor = new_position;
        self.check_cursor_position();
    }

    fn push_char(&mut self, c: char, attr: ScreenCharAttr) {
        get_screen_buffer()[self.cursor] = ScreenAttrChar::new(c as u8, attr);
        self.add_cursor_position();
    }

    fn scroll_up_screen_one_line(&mut self) {
        let len = SCREEN_MAX_COL * (SCREEN_MAX_ROW - 1);
        for idx in 0..len {
            get_screen_buffer()[idx] = get_screen_buffer()[idx + SCREEN_MAX_COL];
        }
        for idx in 0..SCREEN_MAX_COL {
            get_screen_buffer()[len + idx] = ScreenAttrChar::default();
        }
        if self.cursor > SCREEN_MAX_COL {
            self.cursor = 0;
        } else {
            self.cursor = self.cursor - SCREEN_MAX_COL;
        }
    }

    fn handle_char(&mut self, c: char, attr: ScreenCharAttr) {
        match c {
            ' ' ..= '~' => self.push_char(c, attr),
            '\r' => {
                let line_start = self.cursor / SCREEN_MAX_COL * SCREEN_MAX_COL;
                self.subtract_cursor_position(line_start);
            }
            '\n' => self.line_break(),
            '\x08' => {
                // BS
                if self.cursor > 0 {
                    self.subtract_cursor_position(self.cursor - 1);
                }
            }
            _ => {}
        }
    }
}

impl ScreenDriver for DefaultScreenDriver {
    fn get_cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, index: usize) {
        if index >= SCREEN_BUFFER_LEN {
            return;
        }
        self.cursor = index;
        let crt_address_register = ByteWriteByteReadPort::new(0x3d4);
        let crt_data_register = ByteWriteByteReadPort::new(0x3d5);
        let cursor_low: u8 = (self.cursor & 0xff).try_into().unwrap();
        let cursor_high: u8 = ((self.cursor >> 8) & 0xff).try_into().unwrap();
        // let cursor_low: u8 = 1;
        // let cursor_high: u8 = 1;
        crt_address_register.write(0xe);
        crt_data_register.write(cursor_high);
        crt_address_register.write(0xf);
        crt_data_register.write(cursor_low);
    }

    fn print_char(&mut self, ch: char, attr: ScreenCharAttr) {
        self.handle_char(ch, attr);
        self.set_cursor(self.cursor);
    }

    fn print_str(&mut self, text: &str, attr: ScreenCharAttr) {
        let mut iter = text.chars();
        let mut current: char;
        let mut next: Option<char>;
        if let Some(c) = iter.next() {
            current = c;
        } else {
            return;
        }
        loop {
            next = iter.next();
            if let Some(c) = next {
                if current == '\r' && c == '\n' {
                    current = c;
                    continue;
                }
            }
            self.handle_char(current, attr);
            if let Some(c) = next {
                current = c;
            } else {
                break;
            }
        }
        self.set_cursor(self.cursor);
    }

    fn get_char(&self, index: usize) -> Option<ScreenAttrChar> {
        if index >= SCREEN_BUFFER_LEN {
            return None;
        }
        Some(get_screen_buffer()[index])
    }
}