use core::ffi::c_char;

use wdk_sys::{
    ntddk::{KeGetCurrentIrql, ZwCreateFile, ZwWriteFile},
    FILE_APPEND_DATA, FILE_ATTRIBUTE_NORMAL, FILE_OPEN_IF, FILE_SHARE_VALID_FLAGS,
    FILE_SYNCHRONOUS_IO_NONALERT, IO_STATUS_BLOCK, NT_SUCCESS, OBJECT_ATTRIBUTES,
    OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE, SYNCHRONIZE,
};

use crate::{string::str_to_unicode_string, wrap::handle::Handle};

extern crate alloc;

pub struct File {
    file_handle: Handle,
}

impl File {
    pub fn new(file: &str) -> Result<Self, alloc::string::String> {
        if unsafe { KeGetCurrentIrql() } != 0 {
            return Err(alloc::string::String::from("Error IRQL to Access File"));
        }

        let mut h = Handle::default();

        let mut oa = OBJECT_ATTRIBUTES {
            ObjectName: &mut str_to_unicode_string(file),
            Attributes: OBJ_CASE_INSENSITIVE | OBJ_KERNEL_HANDLE,
            Length: core::mem::size_of::<OBJECT_ATTRIBUTES>() as _,
            ..Default::default()
        };

        let mut io_status = IO_STATUS_BLOCK::default();

        let status: i32 = unsafe {
            ZwCreateFile(
                h.as_ptr(),
                FILE_APPEND_DATA | SYNCHRONIZE, // SYNCHRONIZE
                &mut oa as *mut OBJECT_ATTRIBUTES,
                &mut io_status as *mut IO_STATUS_BLOCK,
                core::ptr::null_mut(),
                FILE_ATTRIBUTE_NORMAL,
                FILE_SHARE_VALID_FLAGS,
                FILE_OPEN_IF,
                FILE_SYNCHRONOUS_IO_NONALERT, // FILE_SYNCHRONOUS_IO_NONALERT,
                core::ptr::null_mut(),
                0,
            )
        };

        if !NT_SUCCESS(status) {
            return Err(alloc::format!(
                "CreateFile Error {:X},{:X}",
                status,
                unsafe { io_status.__bindgen_anon_1.Status }
            ));
        }

        Ok(Self { file_handle: h })
    }

    pub fn write(&mut self, text: *mut c_char, length: u32) -> Result<(), alloc::string::String> {
        if self.file_handle.is_null() {
            return Err(alloc::string::String::from("file_handle is null"));
        }

        let mut io_status = IO_STATUS_BLOCK::default();

        let status = unsafe {
            ZwWriteFile(
                self.file_handle.as_raw(),
                core::ptr::null_mut(),
                Option::None,
                core::ptr::null_mut(),
                &mut io_status as *mut IO_STATUS_BLOCK,
                text as _,
                length,
                0 as _,
                core::ptr::null_mut(),
            )
        };

        if !NT_SUCCESS(status) {
            return Err(alloc::format!(
                "ZwWriteFile Error {:X},{:X}",
                status,
                unsafe { io_status.__bindgen_anon_1.Status }
            ));
        }

        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {}
}
