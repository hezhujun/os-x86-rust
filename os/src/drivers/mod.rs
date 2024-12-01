pub mod screen;
pub mod chardev;
pub mod keyboard;
pub mod rtc;
pub mod tsc;

use core::arch::asm;

pub use chardev::UART;
pub use screen::*;
pub use rtc::*;
pub use tsc::*;

pub fn init() {
    screen::init();
    keyboard::init();
    read_cpu_info();
    if let Some(frequency) = get_tsc_frequency() {
        info!("tsc frequency {}", frequency);
    }
}

pub fn cpuid(eax: u32) -> (u32, u32, u32, u32) {
    let mut eax = eax;
    let mut ebx = 0u32;
    let mut ecx = 0u32;
    let mut edx = 0u32;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") eax,
            out("ebx") ebx,
            out("ecx") ecx,
            out("edx") edx,
        )
    }

    (eax, ebx, ecx, edx)
}

pub fn read_cpu_info() {
    // 获取基本 CPU 信息
    let eax = 0; // EAX = 0 to get CPU vendor
    let (eax, ebx, ecx, edx) = cpuid(eax);
    unsafe {
        info!(
            "Vendor ID: {}{}{}\n", 
            core::str::from_utf8_unchecked(&ebx.to_le_bytes()), 
            core::str::from_utf8_unchecked(&ecx.to_le_bytes()), 
            core::str::from_utf8_unchecked(&edx.to_le_bytes()));
    }
    
    // 获取 CPU 特性
    let eax = 1; // EAX = 1 for CPU features
    let (eax, ebx, ecx, edx) = cpuid(eax);
    info!(
        "Family: {}, Model: {}, Stepping: {}\n",
        (eax >> 8) & 0xF, 
        (eax >> 4) & 0xF, 
        eax & 0xF
    );
    info!("Features: {}\n", edx); // 处理器特性位图

    // 获取扩展功能
    let eax = 0x80000000; // EAX = 0x80000000 to get the highest extended function
    let (eax, ebx, ecx, edx) = cpuid(eax);
    info!("Highest Extended Function: {:#x}\n", eax);
}

pub fn get_tsc_frequency() -> Option<u32> {
    let eax: u32 = 0x15;
    let (eax, ebx, ecx, edx) = cpuid(eax);

    if ebx == 0 || ecx == 0 {
        None
    } else {
        Some(ecx * ebx / eax)
    }
}
