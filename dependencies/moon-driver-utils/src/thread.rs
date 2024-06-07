use core::ffi::c_void;

use wdk::println;
use wdk_sys::{ntddk::{KeSetEvent, KeWaitForSingleObject, PsCreateSystemThread, PsTerminateSystemThread}, LARGE_INTEGER, NT_SUCCESS, STATUS_SUCCESS, THREAD_ALL_ACCESS, _KWAIT_REASON::Executive, _MODE::KernelMode};

use crate::{memory::npp::NPP, wrap::{ethread::Ethread, event::Event, handle::Handle}};

pub struct SystemThread<T> {
    wait_start_event: Event,
    stop_event: Event,
    thread_exit_event: Event,
    thread_handle: Handle,
    thread_objet: Ethread,
    thread_fn: ThreadFn<T>,
    thread_args: Option<T>,
    timer: Option<u64>,
}

type ThreadFn<T> = unsafe fn(args: &mut Option<T>);

impl<T> SystemThread<T> {
    pub unsafe extern "C" fn thread_proc(thread: *mut SystemThread<T>) {
        println!("Thread Running");
        let t = &mut *thread;

        KeSetEvent(t.wait_start_event.as_mut_raw(), 0, 0);

        let function = &mut t.thread_fn;
        let args = &mut t.thread_args;

        if t.timer == Option::None {
            function(args);
        }else{
            let mut time = LARGE_INTEGER::default();
            time.QuadPart = -1 * 10 * 1000 * t.timer.unwrap() as i64;
    
            let e = t.stop_event.as_mut_raw();
    
            loop {
                function(args);
    
                // wait exit
                let status = KeWaitForSingleObject(e as _, Executive as _,
                    KernelMode as _, 0, &mut time as _);
    
                // drop or no timer
                if status == STATUS_SUCCESS {
                    break;
                }
            }
        }

        println!("Ready to exit thread");
        KeSetEvent(t.thread_exit_event.as_mut_raw() as _, 0, 0);

        let _ = PsTerminateSystemThread(0);
    }

    pub fn as_mut_raw(&mut self) -> *mut SystemThread<T>{
        self as *mut _
    }

    pub fn new(thread: ThreadFn<T>,args: Option<T>, timer: Option<u64>) -> Result<NPP<Self>, &'static str> {
        let stop_event = Event::new()?;
        let thread_exit_event = Event::new()?;
        let wait_start_event = Event::new()?;

        let r = NPP::new(Self {
            wait_start_event,
            stop_event,
            thread_exit_event,
            thread_handle: Handle::default(),
            thread_objet: Ethread::default(),
            thread_fn: thread,
            thread_args: args,
            timer
        });
        return Ok(r);
    }

    pub fn start(&mut self) -> bool {
        unsafe {
            let t = core::mem::transmute::<
                    unsafe extern "C" fn(*mut SystemThread<T>),
                    unsafe extern "C" fn(*mut c_void),
                >(Self::thread_proc);

            
            let status = PsCreateSystemThread(&mut self.thread_handle as *mut _ as _, THREAD_ALL_ACCESS, 
                core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 
                Some(t), self.as_mut_raw() as _);

            if !NT_SUCCESS(status) {
                println!("KernelThread Create Error:{}",status);
                return false;
            }

            println!("Create Thread Success Wait Thread Start");

            // wait thread satrt
            let _ = KeWaitForSingleObject(self.wait_start_event.as_mut_raw() as _, Executive as _,
            KernelMode as _, 0, core::ptr::null_mut());

            println!("SystemThread start success");

            self.thread_objet = Ethread::from_handle(&mut self.thread_handle as *mut _ as _);
        }
        return true;
    }
}

impl<T> Drop for SystemThread<T> {
    fn drop(&mut self) {
        println!("SystemThread Drop");
        unsafe{
            let  _r = KeSetEvent(self.stop_event.as_mut_raw(), 0, 0);
            let _ = KeWaitForSingleObject(self.thread_exit_event.as_mut_raw() as _, Executive as _,
                KernelMode as _, 0, core::ptr::null_mut());
        }
    }
}