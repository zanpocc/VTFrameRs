use alloc::vec::{self, Vec};
use wdk::println;
use wdk_sys::{ntddk::{IoCreateSymbolicLink, IoDeleteSymbolicLink}, NT_SUCCESS, UNICODE_STRING};

use crate::utils::utils::{__debugbreak, create_unicode_string, string_to_utf16_slice};

pub struct SymbolicLink {
    name: Vec<u16>,
}

impl SymbolicLink {
    pub fn new(name: &str, target: &str) -> Result<Self, & 'static str> {
        // str to u16[]
        let name = string_to_utf16_slice(name);

        // whatever 
        match Self::create(&name,target){
            Ok(_) => {
                Self::delete(&name);
            },
            Err(e) => {
                return Err(e);
            },
        }

        // twice
        match Self::create(&name,target){
            Ok(_) => {},
            Err(e) => {
                return Err(e);
            },
        }

        Ok(Self {
            name,
        })
    }

    pub fn create(name: &Vec<u16>,target: &str) -> Result<(), & 'static str> {
        // Convert the name to UTF-16 and then create a UNICODE_STRING.
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

        Ok(())
    }

    fn delete(name: &Vec<u16>) {
        let mut name_ptr = create_unicode_string(name.as_slice());

        unsafe{
            let status = IoDeleteSymbolicLink(&mut name_ptr);
            if !NT_SUCCESS(status) {
                println!("DeleteSymbolicLink error");
            }
        }
    }
}

impl Drop for SymbolicLink {
    fn drop(&mut self) {
        Self::delete(&self.name);
    }
}