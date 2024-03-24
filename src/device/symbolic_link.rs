use alloc::vec::Vec;
use moon_driver_utils::string::{u16_slice_to_unicode_string, string_to_u16_slice};
use moon_log::{error, info};
use wdk_sys::{ntddk::{IoCreateSymbolicLink, IoDeleteSymbolicLink}, NT_SUCCESS};


pub struct SymbolicLink {
    name: Vec<u16>,
}

impl SymbolicLink {
    pub fn new(name: &str, target: &str) -> Result<Self, & 'static str> {
        // str to u16[]
        let name = string_to_u16_slice(name);

        // whatever 
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
        let mut name_ptr = u16_slice_to_unicode_string(name.as_slice());

        // Convert the target to UTF-16 and then create a UNICODE_STRING.
        let target = string_to_u16_slice(target);
        let mut target_ptr = u16_slice_to_unicode_string(target.as_slice());

        let status = unsafe {
            IoCreateSymbolicLink(&mut name_ptr, &mut target_ptr)
        };

        if !NT_SUCCESS(status) {
            error!("CreateSymbolicLink error:{:X}",status);
            return Err("CreateSymbolicLink error");
        }

        Ok(())
    }

    fn delete(name: &Vec<u16>) {
        let mut name_ptr = u16_slice_to_unicode_string(name.as_slice());

        unsafe{
            let status = IoDeleteSymbolicLink(&mut name_ptr);
            if !NT_SUCCESS(status) {
                error!("DeleteSymbolicLink error:{:X}",status);
            }
        }
    }
}

impl Drop for SymbolicLink {
    fn drop(&mut self) {
        info!("Start Drop symboliclink");
        Self::delete(&self.name);
    }
}