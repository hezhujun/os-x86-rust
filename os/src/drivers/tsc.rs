/// Timestamp Counter

use core::arch::asm;

pub fn rdtsc() -> u64 {
    unsafe {
        let mut tsc_lo: u32 = 0;
        let mut tsc_hi: u32 = 0;
        asm!(
            "rdtsc",
            out("eax") tsc_lo,
            out("edx") tsc_hi,
        );
        let mut tsc: u64 = (tsc_hi as u64) << 32;
        tsc |= tsc_lo as u64;
        tsc
    }
}
