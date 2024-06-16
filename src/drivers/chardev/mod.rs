mod ns16550a;

use lazy_static::{lazy_static, LazyStatic};

pub trait CharDevice {
    fn init(&self);
    fn read(&self) -> u8;
    fn write(&self, ch: u8);
    fn handle_irq(&self);
}

pub type CharDeviceImpl = ns16550a::NS16550a;

lazy_static! {
    pub static ref UART: CharDeviceImpl = CharDeviceImpl::new();
}
