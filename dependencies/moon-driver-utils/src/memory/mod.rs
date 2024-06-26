pub mod npp;
pub mod pp;
pub mod utils;

use core::{
    alloc::Layout,
    fmt,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use wdk_sys::{
    ntddk::{memset, ExAllocatePool, ExFreePool},
    POOL_TYPE,
};

#[derive(Debug)]
pub struct AllocationError;

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory allocation failed")
    }
}

pub struct PoolMemory<T> {
    ptr: NonNull<T>,
}

unsafe impl<T: Send> Send for PoolMemory<T> {}
unsafe impl<T: Sync> Sync for PoolMemory<T> {}

impl<T> PoolMemory<T> {
    fn allocate_memory(value: Option<T>, pool_type: POOL_TYPE) -> Result<Self, AllocationError> {
        // Calculate the layout for the type T
        let layout = Layout::new::<T>();

        // Allocate memory using ExAllocatePool
        let ptr = unsafe { ExAllocatePool(pool_type, layout.size() as _) as *mut T };

        let data = NonNull::new(ptr).ok_or(AllocationError)?;

        // Write the value into the allocated memory
        unsafe {
            memset(ptr as _, 0, layout.size() as _);
            if let Some(v) = value {
                core::ptr::write(ptr, v);
            }
        };

        Ok(Self { ptr: data })
    }

    pub fn new(value: T, pool_type: POOL_TYPE) -> Result<Self, AllocationError> {
        Self::allocate_memory(Some(value), pool_type)
    }

    pub fn new_type(pool_type: POOL_TYPE) -> Result<Self, AllocationError> {
        Self::allocate_memory(None, pool_type)
    }

    fn drop_internel(&self) {
        // Explicitly drop the value first
        unsafe { core::ptr::drop_in_place(self.as_ptr()) };

        // Free the memory using ExFreePool
        unsafe { ExFreePool(self.as_ptr() as *mut _) };
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T> Deref for PoolMemory<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T> DerefMut for PoolMemory<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T> Drop for PoolMemory<T> {
    fn drop(&mut self) {
        self.drop_internel();
    }
}
