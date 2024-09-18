use crate::{arch::x86::ByteReadPort, console::print, utils::ring_buffer::RingBuffer};
use super::scan_code_set::*;

pub struct KeyboardDriver {
    scan_codes: [u8; 6],
    size: usize,

    pub is_left_shift_pressed: bool,
    pub is_right_shift_pressed: bool,
    pub is_left_ctrl_pressed: bool,
    pub is_right_ctrl_pressed: bool,
    pub is_left_alt_pressed: bool,
    pub is_right_alt_pressed: bool,

    char_buffer: RingBuffer<char, 10>,
}

impl KeyboardDriver {
    pub fn new() -> Self {
        Self { 
            scan_codes: [0; 6], 
            size: 0, 
            is_left_shift_pressed: false, 
            is_right_shift_pressed: false, 
            is_left_ctrl_pressed: false, 
            is_right_ctrl_pressed: false, 
            is_left_alt_pressed: false, 
            is_right_alt_pressed: false, 
            char_buffer: RingBuffer::new('\0'),
        }
    }
}

impl KeyboardDriver {
    pub fn handle_intr(&mut self) {
        let port = ByteReadPort::new(0x60);
        let code = port.read();
        self.push(code);
    }

    fn push(&mut self, code: u8) {
        self.scan_codes[self.size] = code;
        self.size += 1;
        self.parse_scan_code();
    }

    fn push_char(&mut self, c: char) {
        if self.char_buffer.is_full() {
            self.char_buffer.pop();
        }
        self.char_buffer.push(c);
        if let Some(c) = self.char_buffer.pop() {
            print!("{}", c);
        }
    }

    fn finish_input(&mut self) {
        self.scan_codes.iter_mut().for_each(|b| *b = 0);
        self.size = 0;
    }

    pub fn pop(&mut self) -> Option<char> {
        self.char_buffer.pop()
    }

    pub fn is_shift_pressed(&self) -> bool {
        self.is_left_shift_pressed || self.is_right_shift_pressed
    }

    pub fn is_ctrl_pressed(&self) -> bool {
        self.is_left_ctrl_pressed || self.is_right_ctrl_pressed
    }

    pub fn is_alt_pressed(&self) -> bool {
        self.is_left_alt_pressed || self.is_right_alt_pressed
    }
}

impl KeyboardDriver {
    fn parse_scan_code(&mut self) {
        let mut index = 0;
        match self.scan_codes[0] {
            0x01 => {
                // ESC pressed
                self.finish_input();
            },
            0x81 => {
                // ESC released
                self.finish_input();
            },
            0x2a => {
                self.is_left_shift_pressed = true;
                self.finish_input();
            },
            0xaa => {
                self.is_left_shift_pressed = false;
                self.finish_input();
            },
            0x36 => {
                self.is_right_shift_pressed = true;
                self.finish_input();
            },
            0xb6 => {
                self.is_right_shift_pressed = false;
                self.finish_input();
            },
            0x1d => {
                self.is_left_ctrl_pressed = true;
                self.finish_input();
            },
            0x9d => {
                self.is_left_ctrl_pressed = false;
                self.finish_input();
            },
            0x38 => {
                self.is_left_alt_pressed = true;
                self.finish_input();
            },
            0xb8 => {
                self.is_left_alt_pressed = false;
                self.finish_input();
            },
            0x02..=0x1c | 0x1e..=0x29 | 0x2b..=0x35 => {
                if self.is_shift_pressed() {
                    let c = KEY_MAP[self.scan_codes[0] as usize - 0x02][1];
                    self.push_char(c);
                } else {
                    let c = KEY_MAP[self.scan_codes[0] as usize - 0x02][0];
                    self.push_char(c);
                }
                self.finish_input();
            },
            0x82..=0x9c | 0x9e..=0xa9 | 0xab..=0xb5 => {
                // released
                self.finish_input();
            },
            0x39 => {
                // space passed
                self.push_char(' ');
                self.finish_input();

            },
            0xb9 => {
                // space released
                self.finish_input();
            },
            0x37 => {
                self.push_char('*');
                self.finish_input();
            },
            0xb7 => {
                self.finish_input();
            },
            0x3a => {
                // caps lock pressed
                self.finish_input();
            },
            0xba => {
                // caps lock released
                self.finish_input();
            },
            0x3b..=0x44 | 0x57..=0x58 => {
                // F1-F12 pressed
                self.finish_input();
            },
            0xbb..=0xc4 | 0xd7..=0xd8 => {
                // F1-F12 released
                self.finish_input();
            },
            0x45 => {
                // num lock pressed
                self.finish_input();
            },
            0xc5 => {
                // num lock released
                self.finish_input();
            },
            0x46 => {
                // scroll lock pressed
                self.finish_input();
            },
            0xc6 => {
                // scroll lock released
                self.finish_input();
            },
            0x47 => {
                // 7home pressed
                self.finish_input();
            },
            0xc7 => {
                // 7home released
                self.finish_input();
            },
            0x48 => {
                // 8up pressed
                self.finish_input();
            },
            0xc8 => {
                // 8up released
                self.finish_input();
            },
            0x49 => {
                // 9PgUp pressed
                self.finish_input();
            },
            0xc9 => {
                // 9PgUp released
                self.finish_input();
            },
            0x4a => {
                // - pressed
                self.push_char('-');
                self.finish_input();
            },
            0xca => {
                // - released
                self.finish_input();
            },
            0x4b => {
                // 4left pressed
                self.finish_input();
            },
            0xcb => {
                // 4left released
                self.finish_input();
            },
            0x4c => {
                // 5 pressed
                self.finish_input();
            },
            0xcc => {
                // 5 released
                self.finish_input();
            },
            0x4d => {
                // 6right pressed
                self.finish_input();
            },
            0xcd => {
                // 6right released
                self.finish_input();
            },
            0x4e => {
                // + pressed
                self.push_char('+');
                self.finish_input();
            },
            0xce => {
                // + released
                self.finish_input();
            },
            0x4f => {
                // end pressed
                self.finish_input();
            },
            0xcf => {
                // end released
                self.finish_input();
            },
            0x50 => {
                // 2down pressed
                self.finish_input();
            },
            0xd0 => {
                // 2down released
                self.finish_input();
            },
            0x51 => {
                // 3PgDn pressed
                self.finish_input();
            },
            0xd1 => {
                // 3PgDn released
                self.finish_input();
            },
            0x52 => {
                // 0Ins pressed
                self.finish_input();
            },
            0xd2 => {
                // 0Ins released
                self.finish_input();
            },
            0x53 => {
                // .del pressed
                self.finish_input();
            },
            0xd3 => {
                // .del released
                self.finish_input();
            },
            0xe0 => {
                self.parse_ext_scan_code();
            },
            _ => {
                assert!(false, "Unhandle scan code {}", self.scan_codes[0]);
            },
        }
    }

    fn parse_ext_scan_code(&mut self) {
        match self.scan_codes {
            KEY_RIGHT_ALT_PRESSED => {
                self.is_right_alt_pressed = true;
                self.finish_input();
            },
            KEY_RIGHT_ALT_RELEASED => {
                self.is_right_alt_pressed = false;
                self.finish_input();
            },
            KEY_RIGHT_CTRL_PRESSED => {
                self.is_right_ctrl_pressed = true;
                self.finish_input();
            },
            KEY_RIGHT_CTRL_RELEASED => {
                self.is_right_ctrl_pressed = false;
                self.finish_input();
            },
            KEY_PRINT_SCREEN_PRESSED => {
                self.finish_input();
            },
            KEY_PRINT_SCREEN_RELEASED => {
                self.finish_input();
            },
            KEY_PAUSE_BREAK_PRESSED => {
                self.finish_input();
            },
            KEY_INSERT_PRESSED => {
                self.finish_input();
            },
            KEY_INSERT_RELEASED => {
                self.finish_input();
            },
            KEY_FORWARD_SLASH_EXT_PRESSED => {
                self.finish_input();
            },
            KEY_FORWARD_SLASH_EXT_RELEASED => {
                self.finish_input();
            },
            KEY_HOME_PRESSED => {
                self.finish_input();
            },
            KEY_HOME_RELEASED => {
                self.finish_input();
            },
            KEY_PAGE_UP_PRESSED => {
                self.finish_input();
            },
            KEY_PAGE_UP_RELEASED => {
                self.finish_input();
            },
            KEY_DELETE_PRESSED => {
                self.finish_input();
            },
            KEY_DELETE_RELEASED => {
                self.finish_input();
            },
            KEY_END_PRESSED => {
                self.finish_input();
            },
            KEY_END_RELEASED => {
                self.finish_input();
            },
            KEY_PAGE_DOWN_PRESSED => {
                self.finish_input();
            },
            KEY_PAGE_DOWN_RELEASED => {
                self.finish_input();
            },
            KEY_LEFT_PRESSED => {
                self.finish_input();
            },
            KEY_LEFT_RELEASED => {
                self.finish_input();
            },
            KEY_RIGHT_PRESSED => {
                self.finish_input();
            },
            KEY_RIGHT_RELEASED => {
                self.finish_input();
            },
            KEY_UP_PRESSED => {
                self.finish_input();
            },
            KEY_UP_RELEASED => {
                self.finish_input();
            },
            KEY_DOWN_PRESSED => {
                self.finish_input();
            },
            KEY_DOWN_RELEASED => {
                self.finish_input();
            },
            KEY_ENTER_EXT_PRESSED => {
                self.finish_input();
            },
            KEY_ENTER_EXT_RELEASED => {
                self.finish_input();
            },
            _ => {
                if self.size == 6 {
                    assert!(false);
                    self.finish_input();
                }
            }
        }
    }
}

pub const KEY_MAP: [[char; 2]; 52] = [
    ['1', '!'],  // 0x02
    ['2', '@'],  // 0x03
    ['3', '#'],  // 0x04
    ['4', '$'],  // 0x05
    ['5', '%'],  // 0x06
    ['6', '^'],  // 0x07
    ['7', '&'],  // 0x08
    ['8', '*'],  // 0x09
    ['9', '('],  // 0x0a
    ['0', ')'],  // 0x0b
    ['-', '_'],  // 0x0c
    ['=', '+'],  // 0x0d
    ['\x08', '\x08'],  // 0x0e
    ['\t', '\t'],  // 0x0f
    ['q', 'Q'],  // 0x10
    ['w', 'W'],  // 0x11
    ['e', 'E'],  // 0x12
    ['r', 'R'],  // 0x13
    ['t', 'T'],  // 0x14
    ['y', 'Y'],  // 0x15
    ['u', 'U'],  // 0x16
    ['i', 'I'],  // 0x17
    ['o', 'O'],  // 0x18
    ['p', 'P'],  // 0x19
    ['[', '{'],  // 0x1a
    [']', '}'],  // 0x1b
    ['\r', '\r'],  // 0x1c
    ['\0', '\0'],
    ['a', 'A'], // 0x1e
    ['s', 'S'], // 0x1f
    ['d', 'D'], // 0x20
    ['f', 'F'], // 0x21
    ['g', 'G'], // 0x22
    ['h', 'H'], // 0x23
    ['j', 'J'], // 0x24
    ['k', 'K'], // 0x25
    ['l', 'L'], // 0x26
    [';', ':'], // 0x27
    ['\'', '"'], // 0x28
    ['`', '~'], // 0x29
    ['\0', '\0'],
    ['\\', '|'], // 0x2b
    ['z', 'Z'], // 0x2c
    ['x', 'X'], // 0x2d
    ['c', 'C'], // 0x2e
    ['v', 'V'], // 0x2f
    ['b', 'B'], // 0x30
    ['n', 'N'], // 0x31
    ['m', 'M'], // 0x32
    [',', '<'], // 0x33
    ['.', '>'], // 0x34
    ['/', '?'], // 0x35
];
