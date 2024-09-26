use crate::drivers::{keyboard::get_char, screen_print};

use super::File;
use crate::screen_print;
use crate::schedule::suspend_current_and_run_next;

pub struct Stdin;
pub struct Stdout;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        if buf.len() == 0 {
            return 0;
        }
        let c: u8;
        loop {
            if let Some(ch) = get_char() {
                c = ch;
                break;
            } else {
                suspend_current_and_run_next();
            }
        }
        buf[0] = c;
        1
    }

    fn write(&self, buf: &[u8]) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        panic!("Cannot read from stdout!");
    }

    fn write(&self, buf: &[u8]) -> usize {
        match core::str::from_utf8(buf) {
            Ok(str) => {
                screen_print!("{}", str);
                buf.len()
            },
            Err(err) => {
                error!("{}", err);
                0
            },
        }
    }
}