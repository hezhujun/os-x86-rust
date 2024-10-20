pub mod screen;
pub mod chardev;
pub mod keyboard;
pub mod rtc;
pub mod tsc;

pub use chardev::UART;
pub use screen::*;
pub use rtc::*;
pub use tsc::*;

pub fn init() {
    screen::init();
    keyboard::init();
}