#![no_std]

use string::str_to_unicode_string;
use wdk_sys::{OBJECT_ATTRIBUTES, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE};

pub mod string;
pub mod bitfield;
pub mod registry;
pub mod memory;
pub mod file;
pub mod timer;
pub mod mutex;
pub mod spinlock;
pub mod os_version;

pub fn init_obj_attr(oa: &mut OBJECT_ATTRIBUTES,name: &str) {
    oa.ObjectName = &mut str_to_unicode_string(name);
    oa.Attributes = OBJ_CASE_INSENSITIVE | OBJ_KERNEL_HANDLE;
    oa.Length = core::mem::size_of::<OBJECT_ATTRIBUTES>() as _;
}