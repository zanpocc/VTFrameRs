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

        // Write the value into the allocated memory
        unsafe { 
            memset(ptr as _, 0, layout.size() as _);
            core::ptr::write(ptr, value) 
        };
        
        PP { ptr }
    }

    pub fn drop_internel(&self) {
        // Explicitly drop the value first
        unsafe { core::ptr::drop_in_place(self.as_raw()) };
                
        // Free the memory using ExFreePool
        unsafe { ExFreePool(self.as_raw() as *mut _) };
    }

    pub fn as_raw(&self) -> *mut T {
        self.ptr
    }

    // drop by youself
    pub fn into_raw(&mut self) -> *mut T{
        unsafe { core::ptr::replace(&mut self.ptr, core::ptr::null_mut()) }
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
        self.drop_internel();
    }
}
