use core::{alloc::Layout, ops::{Deref, DerefMut}};

use wdk_sys::{ntddk::{memset, ExAllocatePool, ExFreePool}, _POOL_TYPE::PagedPool};

pub struct PP<T> {
    ptr: *mut T,
}

unsafe impl<T: Send> Send for PP<T> {}
unsafe impl<T: Sync> Sync for PP<T> {}

impl<T> PP<T> {
    pub fn new(value: T) -> Self {
        // Calculate the layout for the type T
        let layout = Layout::new::<T>();
        
        // Allocate memory using ExAllocatePool
        let ptr = unsafe { 
            ExAllocatePool(PagedPool, layout.size() as _) as *mut T 
        };

        if ptr.is_null(){
            panic!("Error to ExAllocatePool PagedPool");
        }

        // Write the value into the allocated memory
        unsafe {
            memset(ptr as _, 0, layout.size() as _);
            core::ptr::write(ptr, value) 
        };

        PP { ptr }
    }

    pub fn get(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Deref for PP<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for PP<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for PP<T> {
    fn drop(&mut self) {
        // Explicitly drop the value first
        unsafe { core::ptr::drop_in_place(self.get()) };
        
        // Free the memory using ExFreePool
        unsafe { ExFreePool(self.get() as *mut _) };
    }
}
