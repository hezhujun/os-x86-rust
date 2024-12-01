use crate::arch::x86::{inb, outb, ByteReadPort, ByteWritePort};

const RTC_PORT: u16 = 0x70;
const RTC_DATA_PORT: u16 = 0x71;

pub fn read_second() -> u8 {
    outb(0x00, RTC_PORT);
    let value = inb(RTC_DATA_PORT);
    if is_rtc_bcd_format() {
        bcd_to_decimal(value)
    } else {
        value
    }
}

pub fn read_minute() -> u8 {
    outb(0x02, RTC_PORT);
    let value = inb(RTC_DATA_PORT);
    if is_rtc_bcd_format() {
        bcd_to_decimal(value)
    } else {
        value
    }
}

pub fn read_hour() -> u8 {
    outb(0x04, RTC_PORT);
    let value = inb(RTC_DATA_PORT);
    if is_rtc_bcd_format() {
        bcd_to_decimal(value)
    } else {
        value
    }
}

pub fn read_day() -> u8 {
    outb(0x07, RTC_PORT);
    let value = inb(RTC_DATA_PORT);
    if is_rtc_bcd_format() {
        bcd_to_decimal(value)
    } else {
        value
    }
}

pub fn read_mouth() -> u8 {
    outb(0x08, RTC_PORT);
    let value = inb(RTC_DATA_PORT);
    if is_rtc_bcd_format() {
        bcd_to_decimal(value)
    } else {
        value
    }
}

pub fn read_year() -> u8 {
    outb(0x09, RTC_PORT);
    let value = inb(RTC_DATA_PORT);
    if is_rtc_bcd_format() {
        bcd_to_decimal(value)
    } else {
        value
    }
}

pub fn is_rtc_bcd_format() -> bool {
    outb(0x0B, RTC_PORT);
    let reg_b = inb(RTC_DATA_PORT);
    reg_b & (1 << 5) == 0 // 0 表示 BCD，1 表示二进制
}

pub fn bcd_to_decimal(bcd: u8) -> u8 {
    return (bcd >> 4) * 10 + (bcd & 0x0F); // 高四位乘以 10，加上低四位
}
