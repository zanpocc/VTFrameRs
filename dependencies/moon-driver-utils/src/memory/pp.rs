use core::ops::{Deref, DerefMut};

use wdk_sys::_POOL_TYPE::PagedPool;

use super::{AllocationError, PoolMemory};

pub struct PP<T> {
    m: PoolMemory<T>,
}

unsafe impl<T: Send> Send for PP<T> {}
unsafe impl<T: Sync> Sync for PP<T> {}

impl<T> PP<T> {
    pub fn new(value: T) -> Result<Self, AllocationError> {
        let p = PoolMemory::new(value, PagedPool)?;
        Ok(Self { m: p })
    }

    pub fn new_type() -> Result<Self, AllocationError> {
        let p = PoolMemory::new_type(PagedPool)?;
        Ok(Self { m: p })
    }

    pub fn as_ptr(&self) -> *mut T {
        self.m.as_ptr()
    }
}

impl<T> Deref for PP<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr() }
    }
}

impl<T> DerefMut for PP<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_ptr() }
    }
}
