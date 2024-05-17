#![no_std]

pub mod vmx;
pub mod device;
pub mod driver;
pub mod utils;
pub mod gd;
pub mod inner;
pub mod mem;
pub mod hook;
pub mod slib;
pub mod symbol;

extern crate alloc;

// #[cfg(not(test))]
extern crate wdk_panic;

#[macro_use]
extern crate lazy_static;


use alloc::boxed::Box;
use device::{device::Device, ioctl::IoControl, symbolic_link::SymbolicLink};
use driver::driver::Driver;
use hook::inline_hook::InlineHook;
use mem::mem::PageTableTansform;
use moon_driver_utils::memory::{wpoff, wpon};
use moon_log::{error, info};

// #[cfg(not(test))]
use mem::global_alloc::WDKAllocator;


// #[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::{ntddk::memcpy, ACCESS_MASK, DRIVER_OBJECT, IRP_MJ_MAXIMUM_FUNCTION, NTSTATUS, PCLIENT_ID, PCUNICODE_STRING, PDRIVER_OBJECT, PHANDLE64, POBJECT_ATTRIBUTES, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::{device::device::dispatch_device, gd::gd::GD, hook::inline_hook::inline_hook, symbol::{generic::OS_INFO, symbol::get_ssdt_function_by_name}, vmx::{check::check_vmx_cpu_support, vmx::Vmm}};

static mut __GD:Option<Box<GD>> = Option::None;

// pub unsafe extern "C" fn timer_callback(
//     _dpc: *mut KDPC,
//     deferred_context: *mut c_void,
//     _system_argument1: *mut c_void,
//     _system_argument2: *mut c_void) {

//     println!("timer_callback");

//     if deferred_context.is_null(){
//         return;
//     }

//     let log = &mut *(deferred_context as *mut CircularLogBuffer);
//     log.acquire();
//     // todo:can not createfile on irql 2 Dispatch_level
//     // log.persist_to_file();
//     log.release();
// }

type NtOpenProcessFn = unsafe extern "system" fn(
    ProcessHandle: PHANDLE64,
    DesiredAccess: ACCESS_MASK,
    ObjectAttributes: POBJECT_ATTRIBUTES,
    ClientId: PCLIENT_ID,
) -> NTSTATUS;

static mut NEW_OLD_NT_OPEN_PROCESS:Option<InlineHook> = Option::None;

pub unsafe fn my_nt_open_process(process_handle: PHANDLE64,desired_access: ACCESS_MASK,object_attributes: POBJECT_ATTRIBUTES ,client_id: PCLIENT_ID) -> NTSTATUS {
    info!("hello ntopenprocess");

    if let Some(hook) = &NEW_OLD_NT_OPEN_PROCESS {
        let nt_open_process: NtOpenProcessFn = core::mem::transmute(hook.new_ori_func_header);
        return nt_open_process(process_handle, desired_access, object_attributes, client_id);
    }

    // Return an error code if the function pointer is not set
    STATUS_UNSUCCESSFUL
}


#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver_object: PDRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    let status = STATUS_SUCCESS;

    info!("Driver entry");

    let nt_open_process = get_ssdt_function_by_name("NtOpenProcess");
    info!("NtOpenProcess:{:p}",nt_open_process);

    let hook = inline_hook(nt_open_process as _, my_nt_open_process as _);
    match hook {
        Ok(h) => {
            let patch_size = h.patch_size.clone();
            let patch_header = h.patch_header.clone();

            NEW_OLD_NT_OPEN_PROCESS = Some(h);

            let irql = wpoff();
            memcpy(nt_open_process as _,patch_header as _, patch_size as _);
            wpon(irql);
        }
        Err(_) => {
            info!("hook错误");
        }
    }

    __GD = Some(Box::new(GD::default()));

    info!("{}",OS_INFO.version_name);

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

                gd.vmm = Some(Vmm::new());
                match gd.vmm.as_mut().unwrap().start() {
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
        (*driver_object).MajorFunction[i as usize] = Some(dispatch_device);
    }

    (*driver_object).DriverUnload = Some(driver_unload);

    

    status
}

pub unsafe extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
    // clear resources when drvier unload
    __GD.take();
    info!("DriverUnload Success");
}