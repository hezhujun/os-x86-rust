use core::fmt::{self, Write};
use crate::write;
use crate::read;

const STDIN: usize = 0;
const STDOUT: usize = 1;

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    loop {
        let size = read(STDIN, &mut c);
        if size == 1 {
            return c[0]
        }
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}