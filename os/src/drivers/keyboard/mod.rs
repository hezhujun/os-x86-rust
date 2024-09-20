use alloc::sync::Arc;
use keyboard::KeyboardDriver;
use spin::Mutex;

mod keyboard;
mod scan_code_set;

lazy_static! {
    static ref KEYBOARD_DRIVER: Arc<Mutex<keyboard::KeyboardDriver>> = Arc::new(Mutex::new(KeyboardDriver::new()));
}

pub fn init() {

}

pub fn handle_keyboard_intr() {
    let mut keyboard = KEYBOARD_DRIVER.lock();
    keyboard.handle_intr();
}

pub fn get_char() -> Option<u8> {
    let mut keyboard = KEYBOARD_DRIVER.lock();
    keyboard.pop()
}
