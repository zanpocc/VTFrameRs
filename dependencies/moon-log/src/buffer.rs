use alloc::{ffi::CString, format};
use moon_driver_utils::{
    file::File,
    memory::npp::NPP,
    rwlock::{ReadWriteLock, WriteGuard},
    spinlock::ReentrantSpinLock,
    thread::{self, SystemThread},
    time::get_current_time,
};
use wdk::println;
use wdk_sys::{
    ntddk::{memset, strlen},
    SIZE_T,
};

extern crate alloc;

const LOG_BUFFER_SIZE: SIZE_T = 1024;

#[repr(C)]
struct LogEntry {
    length: u32,
    buffer: u8,
}

#[repr(C)]
pub struct CircularLogBuffer {
    offset: u64,
    buffer: NPP<[u8; LOG_BUFFER_SIZE as _]>, // 2mb
    file: File,
    lock: ReentrantSpinLock,
    thread: Option<NPP<SystemThread<LOG>>>,
}

unsafe impl Sync for CircularLogBuffer {}
unsafe impl Send for CircularLogBuffer {}

pub fn log_thread(_args: &mut Option<LOG>) {
    println!("log thread");

    let mut rw = LOG.write();
    if let Some(r) = &mut *rw {
        r.persist_to_file();
    }
}

pub fn drop_log() {
    let mut rw: WriteGuard<Option<NPP<CircularLogBuffer>>> = LOG.write();
    rw.take();
}

lazy_static! {
    pub static ref LOG: ReadWriteLock<Option<NPP<CircularLogBuffer>>> = {
        println!("LOG Init");
        let c = match CircularLogBuffer::new() {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                return ReadWriteLock::new(Option::None);
            }
        };

        let mut r = match NPP::new(c) {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                return ReadWriteLock::new(Option::None);
            }
        };

        match thread::SystemThread::new(log_thread, None, Some(5000)) {
            Ok(mut tt) => {
                tt.start();
                r.thread = Some(tt);
            }
            Err(e) => {
                println!("{}", e);
                return ReadWriteLock::new(Option::None);
            }
        }

        ReadWriteLock::new(Some(r))
    };
}

impl CircularLogBuffer {
    pub fn new() -> Result<Self, alloc::string::String> {
        let buffer = match NPP::<[u8; LOG_BUFFER_SIZE as _]>::new_type() {
            Ok(r) => r,
            Err(_e) => {
                return Err(alloc::string::String::from("allocate memory error"));
            }
        };

        let path = format!("\\??\\C:\\{}.log", get_current_time());
        let file = File::new(&path)?;

        let r = Self {
            offset: 0,
            buffer,
            file,
            lock: ReentrantSpinLock::new(),
            thread: Option::None,
        };

        Ok(r)
    }

    pub fn persist_to_file(&mut self) {
        if self.offset == 0 {
            println!("persist_to_file do nothing");
            return;
        }

        // from 0 to offset
        let mut i = 0;
        while i < self.offset {
            let entry = unsafe { &mut *((self.buffer.as_ptr() as u64 + i) as *mut LogEntry) };
            let length = unsafe { strlen(&mut entry.buffer as *mut u8 as *const i8) } as u32;
            let size = (core::mem::size_of::<LogEntry>() - 1 + length as usize) as u64;

            let r = self
                .file
                .write(&mut entry.buffer as *mut u8 as *mut i8, length);

            if let Err(e) = r {
                println!("{}", e);
            }

            i += size;
        }

        // zero memory
        unsafe { memset(self.buffer.as_ptr() as _, 0, LOG_BUFFER_SIZE) };

        // point to start
        self.offset = 0;
    }

    pub fn write_log(&mut self, args: core::fmt::Arguments) {
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
            let entry = &mut *((self.buffer.as_ptr() as u64 + self.offset) as *mut LogEntry);
            entry.length = length;

            let p = &mut entry.buffer as *mut u8;
            core::ptr::copy(buff as *mut u8, p, length as _);
        }

        // next
        self.offset += size;
    }
}

impl Drop for CircularLogBuffer {
    fn drop(&mut self) {
        println!("CircularLogBuffer drop");
    }
}
