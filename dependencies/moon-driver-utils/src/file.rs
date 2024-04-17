use core::ffi::{c_char, c_void};

use wdk::println;
use wdk_sys::{ntddk::{KeGetCurrentIrql, ZwClose, ZwCreateFile, ZwWriteFile}, FILE_APPEND_DATA, FILE_ATTRIBUTE_NORMAL, FILE_OPEN_IF, FILE_SHARE_READ, FILE_SHARE_VALID_FLAGS, FILE_SYNCHRONOUS_IO_NONALERT, IO_STATUS_BLOCK, NT_SUCCESS, OBJECT_ATTRIBUTES, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE, SYNCHRONIZE};

use crate::string::str_to_unicode_string;



pub struct File {
    file_handle: *mut c_void,
}

impl File {
    pub fn new(file: &str) -> Self {
        if unsafe { KeGetCurrentIrql() } == 0 {
            println!("Error IRQL to Access File");
        }

        let mut r = File{
            file_handle: core::ptr::null_mut()
        };

        let mut oa = OBJECT_ATTRIBUTES::default();
        oa.ObjectName = &mut str_to_unicode_string(file);
        oa.Attributes = OBJ_CASE_INSENSITIVE | OBJ_KERNEL_HANDLE;
        oa.Length = core::mem::size_of::<OBJECT_ATTRIBUTES>() as _;

        let mut io_status = IO_STATUS_BLOCK::default();

        let status = unsafe { 
            ZwCreateFile(
                &mut r.file_handle as *mut *mut c_void ,
                FILE_APPEND_DATA | SYNCHRONIZE, // SYNCHRONIZE 
                &mut oa as *mut OBJECT_ATTRIBUTES, 
                &mut io_status as *mut IO_STATUS_BLOCK, 
                core::ptr::null_mut(), 
                FILE_ATTRIBUTE_NORMAL, 
                FILE_SHARE_READ, 
                FILE_OPEN_IF, 
                FILE_SYNCHRONOUS_IO_NONALERT, // FILE_SYNCHRONOUS_IO_NONALERT, 
                core::ptr::null_mut(), 
                0
            ) 
        };

        if !NT_SUCCESS(status) {
            println!("CreateFile Error {:X},{:X}",status, unsafe{ io_status.__bindgen_anon_1.Status });
        }   
        
        r
    }

    pub fn write(&mut self,text: *mut c_char,length: u32) {
        if self.file_handle.is_null(){
            return;
        }

        let mut io_status = IO_STATUS_BLOCK::default();

        println!("Begin Call ZwWriteFile:{:p},length:{}",text,length);
        let status = unsafe { 
            ZwWriteFile(self.file_handle, 
                core::ptr::null_mut(), 
                Option::None, 
                core::ptr::null_mut(), 
                &mut io_status as *mut IO_STATUS_BLOCK, 
                text as _, 
                length, 
                0 as _, 
                core::ptr::null_mut()
            )
        };

        if !NT_SUCCESS(status) {
            println!("ZwWriteFile Error {:X},{:X}",status, unsafe{ io_status.__bindgen_anon_1.Status });
        }else{
            println!("Call ZwWriteFile Success");
        }

    }
}

impl Drop for File{
    fn drop(&mut self) {
        if !self.file_handle.is_null(){
            println!("close file");
            unsafe { let _ = ZwClose(self.file_handle); };
        }
    }
}