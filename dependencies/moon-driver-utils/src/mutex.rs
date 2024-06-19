use wdk::println;
use wdk_sys::{ntddk::{ExAcquireFastMutex, ExReleaseFastMutex, KeInitializeEvent}, FAST_MUTEX, FM_LOCK_BIT, _EVENT_TYPE::SynchronizationEvent};


pub struct MutexLock {
    mutex: FAST_MUTEX,
    started: bool,
}

fn write_raw(dst:*mut i32, value: i32) {
    unsafe { *dst = value };
}

pub fn ex_initialize_fast_mutex(fast_mutex: *mut FAST_MUTEX) {
    unsafe {
        let fast_mutex = fast_mutex.as_mut().unwrap();
        write_raw(&mut fast_mutex.Count, FM_LOCK_BIT as _);
        fast_mutex.Owner = core::ptr::null_mut();
        fast_mutex.Contention = 0;
        KeInitializeEvent(&mut fast_mutex.Event, SynchronizationEvent, 0);
        return;
    }
}

impl MutexLock {
    pub fn new() -> Self {
        let mut mutex = FAST_MUTEX::default();
        ex_initialize_fast_mutex(&mut mutex);
        
        MutexLock { mutex, started: false }
    }

    pub fn acquire(&mut self) {
        unsafe {
            ExAcquireFastMutex(&mut self.mutex);
            self.started = true;
        }
    }

    pub fn release(&mut self) {
        unsafe {
            ExReleaseFastMutex(&mut self.mutex);
            self.started = false;
        }
    }
}


impl Drop for MutexLock {
    fn drop(&mut self) {
        if self.started {
            println!("Forget to release mutex lock");
            self.release();
        }
    }
}
