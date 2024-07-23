mod ns16550a;

pub trait CharDevice {
    fn init(&self);
    fn read(&self) -> u8;
    fn write(&self, ch: u8);
    fn handle_irq(&self);
}

pub type CharDeviceImpl = ns16550a::NS16550a;

lazy_static! {
    pub static ref UART: CharDeviceImpl = {
        let driver = CharDeviceImpl::new();
        driver.init();
        driver
    };
}
