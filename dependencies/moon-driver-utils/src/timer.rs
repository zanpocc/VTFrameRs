use core::ffi::c_void;

use wdk::println;
use wdk_sys::{
    ntddk::{
        ExAllocatePool, ExFreePool, KeCancelTimer, KeInitializeDpc, KeInitializeTimerEx,
        KeInsertQueueDpc, KeSetTimerEx,
    },
    KDPC, KTIMER, LARGE_INTEGER, PKDEFERRED_ROUTINE,
    _POOL_TYPE::NonPagedPool,
    _TIMER_TYPE::SynchronizationTimer,
};

pub struct Timer {
    timer: *mut KTIMER,
    dpc: *mut KDPC,
    started: bool,
}

impl Timer {
    pub fn new(callback: PKDEFERRED_ROUTINE, arg: *mut c_void) -> Self {
        let mut r = Timer {
            timer: core::ptr::null_mut(),
            dpc: core::ptr::null_mut(),
            started: false,
        };

        unsafe {
            r.timer = ExAllocatePool(NonPagedPool, core::mem::size_of::<KTIMER>() as _) as _;
            r.dpc = ExAllocatePool(NonPagedPool, core::mem::size_of::<KDPC>() as _) as _;
        }

        println!("{:p},{:p}", r.timer, r.dpc);

        unsafe {
            KeInitializeDpc(r.dpc, callback, arg);
            KeInsertQueueDpc(r.dpc, core::ptr::null_mut(), core::ptr::null_mut());
            KeInitializeTimerEx(r.timer, SynchronizationTimer);
        };

        r
    }

    pub fn start(&mut self, sec: i64) {
        if self.started {
            return;
        }

        println!("start timer");

        let mut interval = LARGE_INTEGER::default();
        interval.QuadPart = sec * -10000; // 5 seconds

        unsafe {
            KeSetTimerEx(self.timer, interval, sec as _, self.dpc);
            self.started = true;
        };
    }

    pub fn stop(&mut self) {
        if !self.started {
            return;
        }

        unsafe {
            KeCancelTimer(self.timer);
            self.started = false;
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("drop timer");

        if self.started {
            unsafe {
                KeCancelTimer(self.timer);
            }
        }

        if !self.timer.is_null() {
            unsafe {
                ExFreePool(self.timer as _);
            }
        }

        if !self.dpc.is_null() {
            unsafe {
                ExFreePool(self.dpc as _);
            }
        }
    }
}
