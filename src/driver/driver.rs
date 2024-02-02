use alloc::boxed::Box;
use wdk::println;
use wdk_sys::{ntddk::IoCreateDevice, DRIVER_OBJECT, NT_SUCCESS};

use crate::{device::device::{ DeviceExtension, DeviceOperations, DeviceOperationsVtable}, utils::utils::{create_unicode_string, string_to_utf16_slice}};

use crate::Device;

pub struct Driver{
    pub raw: *mut DRIVER_OBJECT,
}

impl Driver {
    
    pub fn create_device<T: DeviceOperations>(
        &mut self,
        name: &str,
        device_type: u32,
        device_characteristics: u32,
        exclusive: u8,
        data: T
    ) -> Result<Device,&'static str> {

        // Box the data
        let data = Box::new(data);

        let name = string_to_utf16_slice(name);
        let mut name_ptr = create_unicode_string(&name[..]);   


        let mut device = core::ptr::null_mut();

        let status = unsafe{
            IoCreateDevice(
                self.raw,
                core::mem::size_of::<DeviceExtension>() as u32,
                &mut name_ptr,
                device_type,
                device_characteristics,
                exclusive,
                &mut device
            )
        };

        if !NT_SUCCESS(status) {
            return Err("CreateDevice error");
        }

        println!("CreateDevice success");

        let device = unsafe {
            Device::from_raw(device) 
        }; 

        let extension = device.extension_mut();
        extension.vtable = &DeviceOperationsVtable::<T>::VTABLE;
        extension.data = Box::into_raw(data) as *mut cty::c_void;

        Ok(device)

    }
    

    pub unsafe fn from_raw(raw: *mut DRIVER_OBJECT) -> Self {
        Self{
            raw,
        }
    }

    pub unsafe fn as_raw(&self) -> *const DRIVER_OBJECT {
        self.raw as *const _
    }

    pub unsafe fn as_raw_mut(&self) -> *mut DRIVER_OBJECT {
        self.raw
    }

    // 转移所有权，即将当前对象的raw设置为空，并且将原有的raw指针返回
    pub unsafe fn into_raw(mut self) -> *mut DRIVER_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }
}