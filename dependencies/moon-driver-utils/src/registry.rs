extern crate alloc;

use alloc::{slice, string::String};
use wdk::println;
use wdk_sys::{
    ntddk::{ZwClose, ZwOpenKey, ZwQueryValueKey},
    HANDLE, KEY_READ, KEY_VALUE_PARTIAL_INFORMATION, NT_SUCCESS, OBJECT_ATTRIBUTES,
    OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE,
    _KEY_VALUE_INFORMATION_CLASS::KeyValuePartialInformation,
};

use crate::{
    memory::pp::PP,
    string::{str_to_unicode_string, u16_slice_to_string},
};

pub fn query_registry_string(key: &str, value: &str) -> String {
    let len = 1024u32;
    let registry = Registry::new(key, false);
    let pvpi = PP::<[u8; 1024]>::new_type();

    let p = match pvpi {
        Ok(ref pp) => pp.as_ptr(),
        Err(_) => return String::new(),
    };

    let mut length = 0;

    let status = unsafe {
        ZwQueryValueKey(
            registry.h_key,
            &mut str_to_unicode_string(value),
            KeyValuePartialInformation,
            p as _,
            len,
            &mut length,
        )
    };

    if !NT_SUCCESS(status) {
        println!("ZwQueryValueKey error");
        return String::new();
    }

    let temp = unsafe { &mut *(p as *mut KEY_VALUE_PARTIAL_INFORMATION) };
    let p_data = &mut temp.Data as *mut u8 as *mut u16;

    let len = temp.DataLength as usize / 2;

    let buffer_slice = unsafe { slice::from_raw_parts(p_data, len) };

    u16_slice_to_string(buffer_slice)
}

struct Registry {
    h_key: HANDLE,
}

impl Registry {
    // key should begin with \Registry
    pub fn new(key: &str, create: bool) -> Self {
        let mut oa = OBJECT_ATTRIBUTES {
            ObjectName: &mut str_to_unicode_string(key),
            Attributes: OBJ_CASE_INSENSITIVE | OBJ_KERNEL_HANDLE,
            Length: core::mem::size_of::<OBJECT_ATTRIBUTES>() as _,
            ..Default::default()
        };

        // init_obj_attr(&mut oa,key);

        let mut h_key = core::ptr::null_mut();
        if create {
        } else {
            let status = unsafe { ZwOpenKey(&mut h_key, KEY_READ, &mut oa as _) };
            if !NT_SUCCESS(status) {
                println!("openkey error do nothing:{:X}", status);
            }
        }

        Self { h_key }
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        if !self.h_key.is_null() {
            unsafe {
                let _ = ZwClose(self.h_key);
            };
        }
    }
}
