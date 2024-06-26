#![no_std]
pub mod device;
pub mod driver;
pub mod gd;
pub mod hook;
pub mod inner;
pub mod mem;
pub mod slib;
pub mod symbol;
pub mod utils;
pub mod vm;

extern crate alloc;

// #[cfg(not(test))]
extern crate wdk_panic;

#[macro_use]
extern crate lazy_static;

use device::{ioctl::IoControl, symbolic_link::SymbolicLink};
use driver::Driver;
use hook::inline_hook::InlineHook;
use moon_driver_utils::memory::npp::NPP;
use moon_log::{buffer::drop_log, error, info};

// #[cfg(not(test))]
use mem::global_alloc::WDKAllocator;

// #[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use vm::vmx::Vmm;
use wdk_sys::{
    ACCESS_MASK, DRIVER_OBJECT, IRP_MJ_MAXIMUM_FUNCTION, NTSTATUS, PCLIENT_ID, PCUNICODE_STRING,
    PDRIVER_OBJECT, PHANDLE64, POBJECT_ATTRIBUTES, STATUS_SUCCESS, STATUS_UNSUCCESSFUL,
    _DRIVER_OBJECT,
};

use crate::{
    device::dispatch_device,
    gd::GD,
    hook::inline_hook::{NtOpenProcessFn, HOOK_LIST},
    symbol::{generic::OS_INFO, get_ssdt_function_by_name},
    vm::check::check_vmx_cpu_support,
};

static mut __GD: Option<NPP<GD>> = Option::None;

/// # Safety
///
/// this will call origin function
pub unsafe fn my_nt_open_process(
    process_handle: PHANDLE64,
    desired_access: ACCESS_MASK,
    object_attributes: POBJECT_ATTRIBUTES,
    client_id: PCLIENT_ID,
) -> NTSTATUS {
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

pub struct TestError {}

pub fn test() -> Result<(), TestError> {
    // test log persistent
    for i in 0..1000 {
        info!("Test Log:{}", i);
    }

    // test inline hook
    let nt_open_process = get_ssdt_function_by_name("NtOpenProcess");
    info!("NtOpenProcess:{:p}", nt_open_process);

    let hook = InlineHook::new(nt_open_process as _, my_nt_open_process as _);
    match hook {
        Ok(h) => {
            let mut w = HOOK_LIST.write();
            w.as_mut().unwrap().nt_open_process = Some(h);
            w.as_mut().unwrap().nt_open_process.as_mut().unwrap().hook();
        }
        Err(_) => {
            info!("hook error");
            return Err(TestError {});
        }
    }

    Ok(())
}

pub struct InitError {}

/// # Safety
///
/// init will change global var
pub unsafe fn init(driver_object: &mut _DRIVER_OBJECT) -> Result<(), InitError> {
    // allocate global memeory
    match NPP::new(GD::default()) {
        Ok(r) => {
            __GD = Some(r);
        }
        Err(_e) => {
            return Err(InitError {});
        }
    }

    // check cpu support
    if let Err(e) = check_vmx_cpu_support() {
        error!("{}", e);
        return Err(InitError {});
    }

    // create device and symboliclink)
    let mut driver = Driver::from_raw(driver_object);
    match driver.create_device("\\Device\\20240202", 0x22, 0, 0, IoControl {}) {
        Ok(device) => {
            if let Some(gd) = __GD.as_mut() {
                gd.device = Some(device);
                match SymbolicLink::new("\\??\\20240202", "\\Device\\20240202") {
                    Ok(v) => {
                        gd.symbolic_link = Some(v);
                    }
                    Err(e) => {
                        error!("{}", e);
                        return Err(InitError {});
                    }
                }

                gd.vmm = Some(Vmm::new());
                match gd.vmm.as_mut().unwrap().start() {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(InitError {});
                    }
                }
            }
        }
        Err(err) => {
            error!("{}", err);
            return Err(InitError {});
        }
    }

    Ok(())
}

/// # Safety
///
/// DriverEntry
#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver_object: PDRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    info!("Driver entry");

    let status = STATUS_SUCCESS;
    let driver_object = driver_object.as_mut().unwrap();

    if init(driver_object).is_err() {
        clear();
        return STATUS_UNSUCCESSFUL;
    }

    if test().is_err() {
        clear();
        return STATUS_UNSUCCESSFUL;
    }

    info!("{}", OS_INFO.version_name);

    // set dispatch function
    for i in 0..IRP_MJ_MAXIMUM_FUNCTION {
        driver_object.MajorFunction[i as usize] = Some(dispatch_device);
    }

    driver_object.DriverUnload = Some(driver_unload);
    status
}

/// # Safety
///
/// clear memory
pub unsafe fn clear() {
    // clear resources when drvier unload
    let _ = __GD.take();
    let _ = HOOK_LIST.write().take();

    // drop in end
    drop_log();

    info!("DriverUnload Success");
}

/// # Safety
///
/// driver_unload
pub unsafe extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
    clear();
}
