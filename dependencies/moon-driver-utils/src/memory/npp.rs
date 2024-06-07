use core::{alloc::Layout, ops::{Deref, DerefMut}};

use wdk_sys::{ntddk::{memset, ExAllocatePool, ExFreePool}, _POOL_TYPE::NonPagedPool};

pub struct NPP<T> {
    ptr: *mut T,
}

unsafe impl<T: Send> Send for NPP<T> {}
unsafe impl<T: Sync> Sync for NPP<T> {}

impl<T> NPP<T> {
    pub fn new(value: T) -> Self {
        // Calculate the layout for the type T
        let layout = Layout::new::<T>();
        
        // Allocate memory using ExAllocatePool
        let ptr = unsafe { 
            ExAllocatePool(NonPagedPool, layout.size() as _) as *mut T 
        };

        // Write the value into the allocated memory
        unsafe { 
            memset(ptr as _, 0, layout.size() as _);
            core::ptr::write(ptr, value) 
        };
        
        NPP { ptr }
    }

    pub fn new_type() -> Self {
        // Calculate the layout for the type T
        let layout = Layout::new::<T>();
        
        // Allocate memory using ExAllocatePool
        let ptr = unsafe { 
            ExAllocatePool(NonPagedPool, layout.size() as _) as *mut T 
        };

        // Write the value into the allocated memory
        unsafe { 
            memset(ptr as _, 0, layout.size() as _);
        };
        
        NPP { ptr }
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

}

impl<T> Deref for NPP<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for NPP<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for NPP<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null(){
            self.drop_internel();
        }
    }
}
