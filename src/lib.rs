#![no_std]

pub mod vmx;
pub mod device;
pub mod driver;
pub mod utils;
pub mod gd;
pub mod inner;
pub mod mem;

extern crate alloc;

// #[cfg(not(test))]
extern crate wdk_panic;

use core::ffi::c_void;

use device::{device::Device, ioctl::IoControl, symbolic_link::SymbolicLink};
use driver::driver::Driver;
use mem::mem::PageTableTansform;
use moon_driver_utils::timer::Timer;
use moon_log::{buffer::CircularLogBuffer, error, get_logger, info, println};
use moon_struct::check_os_version;

// #[cfg(not(test))]
use wdk_alloc::WDKAllocator;

// #[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::{DRIVER_OBJECT, IRP_MJ_MAXIMUM_FUNCTION, KDPC, NTSTATUS, PCUNICODE_STRING, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::{device::device::dispatch_device, gd::gd::GD, vmx::{check::check_vmx_cpu_support, vmx::Vmm}};

static mut __GD:Option<GD> = Option::None;


static mut T:Option<Timer> = Option::None;

pub unsafe extern "C" fn timer_callback(
    _dpc: *mut KDPC,
    deferred_context: *mut c_void,
    _system_argument1: *mut c_void,
    _system_argument2: *mut c_void) {

    println!("timer_callback");

    if deferred_context.is_null(){
        return;
    }

    let log = &mut *(deferred_context as *mut CircularLogBuffer);
    log.acquire();
    // todo:can not createfile on irql 2 Dispatch_level
    log.persist_to_file();
    log.release();
}

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver_object: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    let status = STATUS_SUCCESS;

    info!("driver entry2");

    __GD = Some(GD::default());

    match check_os_version(){
        Ok(_) => {}
        Err(e) => {
            error!("{}",e);
            __GD.take();
            return STATUS_UNSUCCESSFUL;
        }
    }

    match check_vmx_cpu_support() {
        Ok(_) => {}
        Err(e) => {
            error!("{}",e);
            __GD.take();
            return STATUS_UNSUCCESSFUL;
        }
    }

    match PageTableTansform::new(true) {
        Ok(ptt) => {
            __GD.as_mut().unwrap().ptt = Some(ptt);
        }
        Err(()) => {
            __GD.take();
            return STATUS_UNSUCCESSFUL;
        }
    }

    
    
    let mut driver = Driver::from_raw(driver_object);

    match driver.create_device("\\Device\\20240202", 0x22, 0, 0, IoControl{}) {
        Ok(device) => {
            if let Some(gd) = __GD.as_mut() {
                gd.device = Some(device);
                match SymbolicLink::new("\\??\\20240202", "\\Device\\20240202"){
                    Ok(v) => {
                        gd.symbolic_link = Some(v);
                    },
                    Err(e) => {
                        error!("{}",e);
                        __GD.take();
                        return STATUS_UNSUCCESSFUL;
                    }
                }

                gd.vmx_data = Some(Vmm::new());
                match gd.vmx_data.as_mut().unwrap().start() {
                    Ok(_) => {}
                    Err(_) => {
                        __GD.take();
                        return STATUS_UNSUCCESSFUL;
                    }
                }
            }
        },
        Err(err) => {
            error!("{}", err);
            __GD.take();
            return STATUS_UNSUCCESSFUL;
        }
    }

    // set dispatch function
    for i in 0..IRP_MJ_MAXIMUM_FUNCTION {
        driver_object.MajorFunction[i as usize] = Some(dispatch_device);
    }

    driver_object.DriverUnload = Some(driver_unload);

    
    for i in 0..=30 {
        (*get_logger()).write_log([1u8,1u8,1u8,1u8,1u8], format_args!("hello world {}",i)); 
    }

    let mut t = Timer::new(Some(timer_callback),get_logger() as *mut CircularLogBuffer as *mut c_void);
    t.start(5000);

    T = Some(t);

    status
}

pub unsafe extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
    // clear resources when drvier unload
    __GD.take();
    T.take();

    info!("DriverUnload Success");
}