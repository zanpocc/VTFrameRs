use core::ffi::c_void;

use alloc::ffi::CString;
use moon_driver_utils::{file::File, memory::pp::PP, spinlock::ReentrantSpinLock, thread::{self, SystemThread}};
use wdk::println;
use wdk_sys::{ntddk::{memset, strlen, MmAllocateContiguousMemory, MmFreeContiguousMemory}, PHYSICAL_ADDRESS, SIZE_T};

extern crate alloc;

#[repr(C)]
struct LogEntry {
    length: u32,
    buffer: u8,
}

#[repr(C)]
pub struct CircularLogBuffer {
    offset: u64,
    buffer: *mut u8, // 2mb
    file: File,
    lock: ReentrantSpinLock,
    thread: Option<PP<SystemThread<LOG>>>
}

const LOG_BUFFER_SIZE: SIZE_T = 1024;

unsafe impl Sync for CircularLogBuffer{}
unsafe impl Send for CircularLogBuffer{}

pub unsafe fn log_thread(args: &mut Option<LOG>){
    println!("log thread");
    let _ = args;
    let t = &mut *LOG.as_raw();
    t.persist_to_file();
}

lazy_static!{
    // todo: fix to option type
    pub static ref LOG:PP<CircularLogBuffer> = {
        println!("LOG Init");

        let mut r = PP::new(CircularLogBuffer::new());

        // todo:time config
        let t = thread::SystemThread::new(log_thread, None, Some(5000));

        match t {
            Ok(mut tt) => {
                tt.start();
                r.thread = Some(tt);
            }  
            Err(e) => {
                println!("{}",e);
            }
        }
        
        r
    };
}

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
            // todo:filename config
            file: File::new("\\??\\C:\\rust_driver.log"),
            lock: ReentrantSpinLock::new(),
            thread: Option::None,
        }
    }

    pub fn persist_to_file(&mut self) {
        let _ = self.lock.lock();

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

            self.file.write(&mut  entry.buffer as *mut u8 as *mut i8,length);

            i += size;
        }

        // zero memory
        unsafe { memset(self.buffer as *mut c_void,0, LOG_BUFFER_SIZE) };

        // point to start
        self.offset = 0;
    }

    pub fn write_log(&mut self, args: core::fmt::Arguments) {
        let _ = self.lock.lock();

        let buff = CString::new(alloc::format!("{args}"))
            .expect("CString should be able to be created from a String.");

        let buff = buff.as_ptr();

        let length = unsafe { strlen(buff) } as u32;

        let size = (core::mem::size_of::<LogEntry>() - 1 + length as usize) as u64;

        // overloap
        if self.offset + size >= LOG_BUFFER_SIZE {
            self.persist_to_file();
        }
        
        unsafe {
            let entry = &mut *((self.buffer as u64 + self.offset) as *mut LogEntry);
            entry.length = length;

            let p = &mut entry.buffer as *mut u8;
            core::ptr::copy(buff as *mut u8, p, length as _);
        }

        // next
        self.offset += size;
    }
}

impl Drop for CircularLogBuffer{
    fn drop(&mut self) {
        println!("CircularLogBuffer drop");
        if !self.buffer.is_null(){
            unsafe{
                MmFreeContiguousMemory(self.buffer as _);
            }
        }
    }
}