use core::ops::{Deref, DerefMut};

use wdk_sys::{ntddk::ZwClose, HANDLE};

pub struct Handle {
    raw: HANDLE,
}

impl Default for Handle {
    fn default() -> Self {
        Self{
            raw: core::ptr::null_mut()
        }
    }
}

impl Handle {
    pub fn as_ptr(&mut self) -> *mut HANDLE {
        &mut self.raw as *mut _
    }

    pub fn as_raw(&mut self) -> HANDLE {
        self.raw
    }
}

impl Deref for Handle{
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for Handle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.raw.is_null(){
            unsafe { let _ = ZwClose(self.raw); };
        }
    }
}