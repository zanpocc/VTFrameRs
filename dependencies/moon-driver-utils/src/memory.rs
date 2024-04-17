use core::ffi::c_void;

use wdk_sys::{ntddk::{ExAllocatePool2, ExFreePool}, POOL_FLAG_PAGED};


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
    unsafe { ExAllocatePool2(POOL_FLAG_PAGED, size as _, 2579) }
}

pub fn free_page_pool(p: *mut c_void) {
   unsafe { ExFreePool(p) } 
}