use crate::{config::{CODE_SELECTOR, DATA_SELECTOR}, intr::*, mm::{PhysAddr, VirtAddr, VirtPageNum}};


#[repr(C)]
pub struct TaskContext {
    pub return_address: usize,
    pub esp: usize,
    
    pub es: usize,
    pub ds: usize,
    pub fs: usize,
    pub gs: usize,
    pub ebx: usize,
    pub ebp: usize,
    pub esi: usize,
    pub edi: usize,
}

impl TaskContext {
    pub fn empty() -> Self {
        Self { return_address: 0, esp: 0, es: 0, ds: 0, fs: 0, gs: 0, ebx: 0, ebp: 0, esi: 0, edi: 0 }
    }

    pub fn go_to_intr_return(kstack_top: VirtAddr, intr_context: IntrContext) -> Self {
        let return_address = intr_exit as usize;
        let mut esp = kstack_top.0 - core::mem::size_of::<IntrContext>();
        let stack_top = VirtAddr(esp);
        *stack_top.as_mut() = intr_context;

        // esp point to return address in stack
        esp -= 4;
        Self { return_address: return_address, esp: esp, es: DATA_SELECTOR as usize, ds: DATA_SELECTOR as usize, fs: DATA_SELECTOR as usize, gs: DATA_SELECTOR as usize, ebx: 0, ebp: 0, esi: 0, edi: 0 }
    }
}
