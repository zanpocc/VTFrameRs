use alloc::vec::Vec;
use wdk::println;
use wdk_sys::{ntddk::{IoCreateSymbolicLink, IoDeleteSymbolicLink}, NT_SUCCESS};

use crate::utils::utils::{create_unicode_string, string_to_utf16_slice};

pub struct SymbolicLink {
    name: Vec<u16>,
}

impl SymbolicLink {
    pub fn new(name: &str, target: &str) -> Result<Self, & 'static str> {
        // Convert the name to UTF-16 and then create a UNICODE_STRING.
        let name = string_to_utf16_slice(name);
        let mut name_ptr = create_unicode_string(name.as_slice());

        // Convert the target to UTF-16 and then create a UNICODE_STRING.
        let target = string_to_utf16_slice(target);
        let mut target_ptr = create_unicode_string(target.as_slice());

        let status = unsafe {
            IoCreateSymbolicLink(&mut name_ptr, &mut target_ptr)
        };

        if !NT_SUCCESS(status) {
            return Err("CreateSymbolicLink error");
        }

        println!("CreateSymbolicLink success");

        Ok(Self {
            name,
        })
    }
}

impl Drop for SymbolicLink {
    fn drop(&mut self) {
        let mut name_ptr = create_unicode_string(self.name.as_slice());

        unsafe {
            let status = IoDeleteSymbolicLink(&mut name_ptr);
            if !NT_SUCCESS(status) {
                println!("DeleteSymboliclink error");
            }
        }

        println!("Delete symboliclink success");
    }
}