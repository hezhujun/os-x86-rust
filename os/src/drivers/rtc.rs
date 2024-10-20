use crate::arch::x86::{ByteReadPort, ByteWritePort};


pub fn read_second() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x00);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_second(seconds: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x00);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((seconds & 0xff) as u8);
}

pub fn read_minute() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x01);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_minute(minute: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x01);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((minute & 0xff) as u8);
}

pub fn read_hour() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x02);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_hour(minute: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x02);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((minute & 0xff) as u8);
}

pub fn read_week() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x03);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_week(week: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x03);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((week & 0xff) as u8);
}

pub fn read_day() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x04);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_day(day: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x04);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((day & 0xff) as u8);
}

pub fn read_mouth() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x05);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_mouth(mouth: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x05);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((mouth & 0xff) as u8);
}

pub fn read_year() -> usize {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x06);
    let data_port = ByteReadPort { port: 0x71 };
    data_port.read() as usize
}

pub fn set_year(year: usize) {
    let order_port = ByteWritePort { port: 0x70 };
    order_port.write(0x06);
    let data_port = ByteWritePort { port: 0x71 };
    data_port.write((year & 0xff) as u8);
}