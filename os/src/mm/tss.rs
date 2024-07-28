#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TSS {
    pub last_tss_ptr: usize,
    pub esp0: usize,
    pub ss0: usize,
    pub esp1: usize,
    pub ss1: usize,
    pub esp2: usize,
    pub ss2: usize,
    pub cr3: usize,
    pub eip: usize,
    pub eflags: usize,
    pub eax: usize,
    pub ecx: usize,
    pub edx: usize,
    pub ebx: usize,
    pub esp: usize,
    pub ebp: usize,
    pub esi: usize,
    pub edi: usize,
    pub es: usize,
    pub cs: usize,
    pub ss: usize,
    pub ds: usize,
    pub fs: usize,
    pub gs: usize,
    pub ldt_selector: usize,
    pub(crate) reserve: u16,
    pub io_map_offset: u16,
}

impl TSS {
    pub fn empty() -> Self {
        Self { last_tss_ptr: 0, esp0: 0, ss0: 0, esp1: 0, ss1: 0, esp2: 0, ss2: 0, cr3: 0, eip: 0, eflags: 0, eax: 0, ecx: 0, edx: 0, ebx: 0, esp: 0, ebp: 0, esi: 0, edi: 0, es: 0, cs: 0, ss: 0, ds: 0, fs: 0, gs: 0, ldt_selector: 0, reserve: 0, io_map_offset: 0 }
    }
}
