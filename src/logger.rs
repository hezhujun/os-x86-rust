use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

use crate::screen_println;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                // 红色
                Level::Error => 31,
                // 黄色
                Level::Warn => 93,
                // 蓝色
                Level::Info => 34,
                // 绿色
                Level::Debug => 32,
                // 灰色
                Level::Trace => 90,
            };
            println!("\x1b[{}m[{:>5}][{}] {}\x1b[0m", color, record.level(), record.target(), record.args());
            screen_println!("[{:>5}][{}] {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
}
