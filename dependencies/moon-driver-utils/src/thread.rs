use core::{arch::asm, ffi::c_void};

use wdk::println;
use wdk_sys::{ntddk::{IoCreateNotificationEvent, KeSetEvent, KeWaitForSingleObject, ObfDereferenceObject, PsCreateSystemThread, PsTerminateSystemThread, ZwClose}, HANDLE, NT_SUCCESS, PHANDLE, PKEVENT, PKSTART_ROUTINE, PVOID, THREAD_ALL_ACCESS, _KWAIT_REASON::Executive, _MODE::KernelMode};


pub struct Thread<T> {
    stop_event: PKEVENT,
    stop_event_handle: HANDLE,
    thread_handle: HANDLE,
    thread_fn: ThreadFn<T>,
    thread_args: T
}

type ThreadFn<T> = unsafe fn(args: *mut T);

impl<T> Thread<T> {

    pub unsafe extern "C" fn ThreadProc(thread: *mut Thread<T>) {
        unsafe{

            println!("Start to thread");
            asm!{
                "int 3"
            }
            let t = &mut *thread;
            (t.thread_fn)(&mut t.thread_args as _);
            println!("Start to thread success");

            println!("Start Terminate Sysetm Thread");
            let status = PsTerminateSystemThread(0);
            println!("Terminate Sysetm Thread Error:{}",status);
        }
    }

    pub fn new(thread: ThreadFn<T>,args: T) -> Result<Self,&'static str> {
        let mut handle:HANDLE = core::ptr::null_mut();
        unsafe{
            let event = IoCreateNotificationEvent(core::ptr::null_mut(), &mut handle as _);
            if event.is_null(){
                return Err("IoCreateNotificationEvent Error");
            }

            let r = Self {
                stop_event: event,
                stop_event_handle: handle,
                thread_handle: core::ptr::null_mut(),
                thread_fn: thread,
                thread_args: args,
            };
            return Ok(r);
        }
    }

    pub fn start(&mut self) -> bool {
        unsafe{

            let t = core::mem::transmute::<
                    unsafe extern "C" fn(*mut Thread<T>),
                    unsafe extern "C" fn(*mut c_void),
                >(Self::ThreadProc as unsafe extern "C" fn(*mut Thread<T>));

            let status = PsCreateSystemThread(&mut self.thread_handle as _, THREAD_ALL_ACCESS, 
                core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 
                Some(t), self as *mut Self as _);

            if !NT_SUCCESS(status) {
                println!("KernelThread Create Error:{}",status);
                ObfDereferenceObject(self.thread_handle);
                return false;
            }else{
                println!("KernelThread Create Success");
            }
        }

        return true;
    }

    // pub fn stop(&mut self) {
    //     unsafe{
    //         KeSetEvent(self.stop_event, Increment, Wait)
    //     }
    // }

}

impl<T> Drop for Thread<T> {
    fn drop(&mut self) {
        unsafe{
            asm!{
                "int 3"
            }
        }
        println!("Thread Drop");
        // 确保正确清理资源
        unsafe {
            if !self.thread_handle.is_null() {
                // 发送停止事件
                // 假设你有机制通知线程停止运行
                // KeSetEvent(self.stop_event, 0, 0u8);

                // 等待线程终止
                // let _ = KeWaitForSingleObject(self.thread_handle, Executive, KernelMode as _, 0, core::ptr::null_mut());

                // 关闭线程句柄
                let _ = ZwClose(self.thread_handle);
            }

            if !self.stop_event_handle.is_null() {
                // 关闭事件句柄
                let _ = ZwClose(self.stop_event_handle);
            }
        }
    }
}