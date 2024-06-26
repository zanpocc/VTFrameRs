#![no_std]

use string::str_to_unicode_string;
use wdk_sys::{OBJECT_ATTRIBUTES, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE};

pub mod bitfield;
pub mod file;
pub mod macor;
pub mod memory;
pub mod mutex;
pub mod os_version;
pub mod registry;
pub mod rwlock;
pub mod spinlock;
pub mod string;
pub mod thread;
pub mod time;
pub mod timer;
pub mod wrap;

extern crate lazy_static;

pub fn init_obj_attr(oa: &mut OBJECT_ATTRIBUTES, name: &str) {
    oa.ObjectName = &mut str_to_unicode_string(name);
    oa.Attributes = OBJ_CASE_INSENSITIVE | OBJ_KERNEL_HANDLE;
    oa.Length = core::mem::size_of::<OBJECT_ATTRIBUTES>() as _;
}

#[no_mangle]
#[cfg(test)]
pub extern "C" fn DriverEntry(
    _driver_object: *mut core::ffi::c_void,
    _registry_path: *mut core::ffi::c_void,
) -> i32 {
    -1
}
