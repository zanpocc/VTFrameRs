use core::ops::{Deref, DerefMut};
use wdk_sys::_POOL_TYPE::NonPagedPool;

use super::AllocationError;
use super::PoolMemory;

pub struct NPP<T> {
    m: PoolMemory<T>,
}

unsafe impl<T: Send> Send for NPP<T> {}
unsafe impl<T: Sync> Sync for NPP<T> {}

impl<T> NPP<T> {
    pub fn new(value: T) -> Result<Self, AllocationError> {
        let p = PoolMemory::new(value, NonPagedPool)?;
        Ok(Self { m: p })
    }

    pub fn new_type() -> Result<Self, AllocationError> {
        let p = PoolMemory::new_type(NonPagedPool)?;
        Ok(Self { m: p })
    }

    pub fn as_raw(&self) -> *mut T {
        return self.m.as_raw();
    }
}

impl<T> Deref for NPP<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_raw() }
    }
}

impl<T> DerefMut for NPP<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_raw() }
    }
}
