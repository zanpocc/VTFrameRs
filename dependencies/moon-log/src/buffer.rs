use core::ffi::c_void;

use alloc::ffi::CString;
use moon_driver_utils::{file::File, spinlock::SpinLock};
use wdk_sys::{ntddk::{memset, strlen, MmAllocateContiguousMemory, MmFreeContiguousMemory}, PHYSICAL_ADDRESS, SIZE_T};

use crate::{info, println};

extern crate alloc;

#[repr(C)]
struct LogEntry {
    level: [u8; 5],
    length: u32,
    buffer: u8,
}

#[repr(C)]
pub struct CircularLogBuffer {
    offset: u64,
    buffer: *mut u8, // 2mb
    file: File,
    lock: SpinLock,
}

const LOG_BUFFER_SIZE: SIZE_T = 1024;

unsafe impl Sync for CircularLogBuffer{}
unsafe impl Send for CircularLogBuffer{}

impl CircularLogBuffer {
    pub fn new() -> Self {

        let mut max_size:PHYSICAL_ADDRESS = PHYSICAL_ADDRESS::default();
            max_size.QuadPart = i64::MAX;
            
        let r:*mut u8 = unsafe{ 
            MmAllocateContiguousMemory(LOG_BUFFER_SIZE,max_size) 
        } as _;

        unsafe { memset(r as *mut c_void,0, LOG_BUFFER_SIZE) };

        Self { 
            offset: 0, 
            buffer: r,
            file: File::new("\\??\\C:\\20240330.log"),
            lock: SpinLock::new(),
        }
    }

    pub fn persist_to_file(&mut self) {
        println!("persist_to_file");

        if self.offset == 0 {
            println!("persist_to_file do nothing");
            return;
        }
        
        // from 0 to offset
        let mut i = 0;
        while i < self.offset {
            let entry = unsafe { &mut *((self.buffer as u64 + i) as *mut LogEntry) };
            let length = unsafe { strlen(&mut entry.buffer as *mut u8 as *const i8) } as u32;
            let size = (core::mem::size_of::<LogEntry>() - 1 + length as usize) as u64;

            // println!("p:{:X},offset:{},length:{}",&mut entry.buffer as *mut u8 as u64,i,length);

            self.file.write(&mut  entry.buffer as *mut u8 as *mut i8,length);

            i += size;
        }


        // zero memory
        unsafe { memset(self.buffer as *mut c_void,0, LOG_BUFFER_SIZE) };

        // point to start
        self.offset = 0;
    }

    pub fn acquire(&mut self){
        self.lock.acquire();
    }

    pub fn release(&mut self){
        self.lock.release();
    }

    pub fn write_log(&mut self, level: [u8; 5],args: core::fmt::Arguments) {
        self.acquire();

        let buff = CString::new(alloc::format!("{args}\r\n"))
            .expect("CString should be able to be created from a String.");

        let buff = buff.as_ptr();

        let length = unsafe { strlen(buff) } as u32;
        // info!("length:{}",length);

        let size = (core::mem::size_of::<LogEntry>() - 1 + length as usize) as u64;

        // overloap
        if self.offset + size >= LOG_BUFFER_SIZE {
            // todo:持久化存储
            info!("log buffer overloap");
            self.persist_to_file();
        }
        
        unsafe {
            let entry = &mut *((self.buffer as u64 + self.offset) as *mut LogEntry);
            // println!("p:{:p}",entry);

            entry.level = level;
            entry.length = length;
        
        
            let p = &mut entry.buffer as *mut u8;
            // info!("buffer p:{:X}",p as u64);
            core::ptr::copy(buff as *mut u8, p, length as _);
        }

        // next
        self.offset += size;

        self.release();

        // info!("offset:{}",self.offset);
    }
}


impl Drop for CircularLogBuffer{
    fn drop(&mut self) {
        if !self.buffer.is_null(){
            info!("CircularLogBuffer Drop");
            unsafe{
                MmFreeContiguousMemory(self.buffer as _);
            }
        }
    }
}