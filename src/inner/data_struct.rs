#![allow(non_snake_case)]

use wdk_sys::{_CONTEXT__bindgen_ty_1, CONTEXT, M128A};

#[repr(C)]
#[derive(Default)]
pub struct KDESCRIPTOR {
    pub Pad: [u16; 3],
    pub Limit: u16,
    pub base: u64,
}
pub type PKDESCRIPTOR = *mut KDESCRIPTOR;

#[repr(C)]
#[derive(Default)]
pub struct KSPECIAL_REGISTERS {
    pub Cr0: u64,
    pub Cr2: u64,
    pub Cr3: u64,
    pub Cr4: u64,
    pub KernelDr0: u64,
    pub KernelDr1: u64,
    pub KernelDr2: u64,
    pub KernelDr3: u64,
    pub KernelDr6: u64,
    pub KernelDr7: u64,
    pub Gdtr: KDESCRIPTOR,
    pub Idtr: KDESCRIPTOR,
    pub Tr: u16,
    pub Ldtr: u16,
    pub MxCsr: u32,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
    pub Cr8: u64,
    pub MsrGsBase: u64,
    pub MsrGsSwap: u64,
    pub MsrStar: u64,
    pub MsrLStar: u64,
    pub MsrCStar: u64,
    pub MsrSyscallMask: u64,
    pub Xcr0: u64,
}
pub type PKSPECIAL_REGISTERS = *mut KSPECIAL_REGISTERS;


#[repr(C)]
#[derive(Default)]
pub struct _CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: u32,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub __bindgen_anon_1: _CONTEXT__bindgen_ty_1,
    pub VectorRegister: [M128A; 26usize],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}

#[repr(C)]
#[derive(Default)]
pub struct KPROCESSOR_STATE {
    pub SpecialRegisters: KSPECIAL_REGISTERS,
    pub context_frame: CONTEXT,
}

pub type PKPROCESSOR_STATE = *mut KPROCESSOR_STATE;
