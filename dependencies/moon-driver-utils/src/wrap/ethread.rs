use core::ops::{Deref, DerefMut};

use wdk::println;
use wdk_sys::{
    ntddk::{ObReferenceObjectByHandle, ObfDereferenceObject},
    PsThreadType, NT_SUCCESS, PETHREAD, PHANDLE, THREAD_ALL_ACCESS,
    _MODE::KernelMode,
};

pub struct Ethread {
    raw: PETHREAD,
}

impl Ethread {
    pub fn from_raw(raw: PETHREAD) -> Self {
        Self { raw: raw }
    }

    pub fn from_handle(h: PHANDLE) -> Self {
        let mut r = Self {
            raw: core::ptr::null_mut(),
        };

        unsafe {
            let status = ObReferenceObjectByHandle(
                *h as _,
                THREAD_ALL_ACCESS,
                *PsThreadType,
                KernelMode as _,
                r.as_mut_raw() as _,
                core::ptr::null_mut(),
            );

            if !NT_SUCCESS(status) {
                println!("ObReferenceObjectByHandle thread error:{}", status);
            }
        }

        r
    }

    pub fn as_mut_raw(&mut self) -> *mut PETHREAD {
        &mut self.raw
    }
}

impl Default for Ethread {
    fn default() -> Self {
        Self {
            raw: core::ptr::null_mut(),
        }
    }
}

impl Deref for Ethread {
    type Target = PETHREAD;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for Ethread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl Drop for Ethread {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ObfDereferenceObject(*self.as_mut_raw() as _);
            }
        }
    }
}
