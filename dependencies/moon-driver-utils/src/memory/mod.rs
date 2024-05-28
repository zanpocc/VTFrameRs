pub mod pp;
pub mod npp;

use core::{arch::asm, ffi::c_void};

use wdk_sys::{ntddk::{ExAllocatePool, ExFreePool, KeLowerIrql, KfRaiseIrql}, DISPATCH_LEVEL, KIRQL, SIZE_T, _POOL_TYPE::PagedPool};


pub struct StackPagePoolMemory{
    pub p: *mut c_void
}

impl StackPagePoolMemory{
    pub fn new(size: u32) -> Self{
        Self { p: allocate_page_pool(size as _) }
    }
}

impl Drop for StackPagePoolMemory {
    fn drop(&mut self) {
        if !self.p.is_null(){
            free_page_pool(self.p);
        }
    }
}

pub fn allocate_page_pool(size: u64) -> *mut c_void{
    // unsafe { ExAllocatePool2(POOL_FLAG_PAGED, size as _, 2579) }
    unsafe{ ExAllocatePool(PagedPool, size as SIZE_T) }
}

pub fn free_page_pool(p: *mut c_void) {
   unsafe { ExFreePool(p) } 
}

pub fn wpoff() -> KIRQL {
    unsafe{ 
        let irql = KfRaiseIrql(DISPATCH_LEVEL as _);
        asm!{
            "push rax",
            "mov rax,cr0",
            "and rax,0xfffffffffffeffff",
            "mov cr0,rax",
            "pop rax",

            "cli",
        };
        return irql;
    }
}

pub fn wpon(irql:KIRQL) {
    unsafe{ 
        asm!{
            "mov rax,cr0",
            "or rax,0x10000",
            "sti",
            "mov cr0,rax",
        };
        KeLowerIrql(irql);
    }
}