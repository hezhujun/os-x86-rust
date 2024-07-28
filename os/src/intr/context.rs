use crate::config::*;
use crate::{arch::x86::Eflags, mm::VirtAddr};


#[derive(Clone, Copy)]
#[repr(C)]
pub struct IntrContext {
    pub magic: usize,
    pub es: usize,
    pub ds: usize,
    pub fs: usize,
    pub gs: usize,
    pub eax: usize,
    pub ecx: usize,
    pub edx: usize,
    pub ebx: usize,
    pub ebp: usize,
    pub esi: usize,
    pub edi: usize,
    pub intr: usize,
    pub error_code: usize,
    pub eip: usize,
    pub cs: usize,
    pub eflags: usize,
    pub esp: usize,
    pub ss: usize,
}

impl IntrContext {
    pub fn empty() -> Self {
        Self { magic: 0, es: 0, ds: 0, fs: 0, gs: 0, eax: 0, ecx: 0, edx: 0, ebx: 0, ebp: 0, esi: 0, edi: 0, intr: 0, error_code: 0, eip: 0, cs: 0, eflags: 0, esp: 0, ss: 0}
    }

    pub fn new(register_context: IntrRegisterContext, intr: usize, error_code: usize, eip: usize, cs: usize, eflags: usize, esp: usize, ss: usize) -> Self {
        Self { 
            magic: 0x1234,
            es: register_context.es,
            ds: register_context.ds,
            fs: register_context.fs,
            gs: register_context.gs,
            eax: register_context.eax,
            ecx: register_context.ecx,
            edx: register_context.edx,
            ebx: register_context.ebx,
            ebp: register_context.ebp, 
            esi: register_context.esi, 
            edi: register_context.edi, 
            intr, 
            error_code, 
            eip, 
            cs, 
            eflags,
            esp,
            ss,
        }
    }

    /// Create intr context which will return to run kernel function
    pub fn kernel_intr_context(func_address: VirtAddr) -> Self {
        Self::new(
            IntrRegisterContext::kernel_default(), 
            0, 
            0, 
            func_address.0,  
            CODE_SELECTOR as usize, 
            Eflags::IF.bits() as usize,
            0,
            0,
        )
    }

    /// Create intr context which will return to run user function
    pub fn user_intr_context(func_address: VirtAddr, user_stack_top: VirtAddr) -> Self {
        Self::new(
            IntrRegisterContext::user_default(), 
            0, 
            0, 
            func_address.0,  
            USER_CODE_SELECTOR as usize, 
            Eflags::IF.bits() as usize,
            user_stack_top.0,
            USER_DATA_SELECTOR as usize,
        )
    }
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct IntrRegisterContext {
    pub es: usize,
    pub ds: usize,
    pub fs: usize,
    pub gs: usize,
    pub eax: usize,
    pub ecx: usize,
    pub edx: usize,
    pub ebx: usize,
    pub ebp: usize,
    pub esi: usize,
    pub edi: usize,
}

impl IntrRegisterContext {
    pub fn empty() -> Self {
        Self { es: 0, ds: 0, fs: 0, gs: 0, eax: 0, ecx: 0, edx: 0, ebx: 0, ebp: 0, esi: 0, edi: 0 }
    }

    pub fn kernel_default() -> Self {
        Self { es: DATA_SELECTOR as usize, ds: DATA_SELECTOR as usize, fs: DATA_SELECTOR as usize, gs: DATA_SELECTOR as usize, eax: 0, ecx: 0, edx: 0, ebx: 0, ebp: 0, esi: 0, edi: 0 }
    }
    
    pub fn user_default() -> Self {
        Self { es: DATA_SELECTOR as usize, ds: USER_DATA_SELECTOR as usize, fs: DATA_SELECTOR as usize, gs: DATA_SELECTOR as usize, eax: 0, ecx: 0, edx: 0, ebx: 0, ebp: 0, esi: 0, edi: 0 }
    }
}
