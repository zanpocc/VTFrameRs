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

use device::{device::Device, ioctl::IoControl, symbolic_link::SymbolicLink};
use driver::driver::Driver;
use hook::inline_hook::InlineHook;
use moon_driver_utils::memory::npp::NPP;
use moon_log::{buffer::drop_log, error, info};

// #[cfg(not(test))]
use mem::global_alloc::WDKAllocator;


// #[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::{ACCESS_MASK, DRIVER_OBJECT, IRP_MJ_MAXIMUM_FUNCTION, NTSTATUS, PCLIENT_ID, PCUNICODE_STRING, PDRIVER_OBJECT, PHANDLE64, POBJECT_ATTRIBUTES, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::{device::device::dispatch_device, gd::gd::GD, hook::inline_hook::{NtOpenProcessFn, HOOK_LIST}, symbol::{generic::OS_INFO, symbol::get_ssdt_function_by_name}, vmx::{check::check_vmx_cpu_support, vmx::Vmm}};

static mut __GD:Option<NPP<GD>> = Option::None;

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


pub unsafe fn my_nt_open_process(process_handle: PHANDLE64,desired_access: ACCESS_MASK,object_attributes: POBJECT_ATTRIBUTES ,client_id: PCLIENT_ID) -> NTSTATUS {
    // info!("hello ntopenprocess");
    let r = HOOK_LIST.read();
    
    let r = &r.as_ref();
    let r = r.unwrap();
    let r = r.nt_open_process.as_ref();

    if let Some(hook) = r {
        let nt_open_process: NtOpenProcessFn = core::mem::transmute(hook.new_ori_func_header);
        return nt_open_process(process_handle, desired_access, object_attributes, client_id);
    }

    // Return an error code if the function pointer is not set
    STATUS_UNSUCCESSFUL
}

pub unsafe fn test() {
    for i in 0..1000 {
        info!("Test Log:{}",i);
    }
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

    test();

    let hook = InlineHook::inline_hook(nt_open_process as _, my_nt_open_process as _);
    match hook {
        Ok(h) => {
            let mut w = HOOK_LIST.write();
            (&mut w).as_mut().unwrap().nt_open_process = Some(h);
            (&mut w).as_mut().unwrap().nt_open_process.as_mut().unwrap().hook();
        }
        Err(_) => {
            info!("hook error");
        }
    }

    __GD = Some(NPP::new(GD::default()));

    info!("{}",OS_INFO.version_name);

    match check_vmx_cpu_support() {
        Ok(_) => {}
        Err(e) => {
            error!("{}",e);
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
    let _ = __GD.take();
    let _ = HOOK_LIST.write().take();
    
    info!("DriverUnload Success");

    // drop in end
    drop_log();
}