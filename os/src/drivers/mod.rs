pub mod screen;
pub mod chardev;
pub mod keyboard;

pub use chardev::UART;
pub use screen::*;

pub fn init() {
    keyboard::init();
}