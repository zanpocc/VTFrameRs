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

use device::{device::Device, ioctl::IoControl, symbolic_link::SymbolicLink};
use driver::driver::Driver;
use mem::mem::PageTableTansform;
use moon_struct::check_os_version;
use wdk::println;
// #[cfg(not(test))]
use wdk_alloc::WDKAllocator;

// #[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::{DRIVER_OBJECT, IRP_MJ_MAXIMUM_FUNCTION, NTSTATUS, PCUNICODE_STRING, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::{device::device::dispatch_device, gd::gd::GD, vmx::{check::check_vmx_cpu_support, vmx::Vmm}};

static mut __GD:Option<GD> = Option::None;

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver_object: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    let status = STATUS_SUCCESS;

    __GD = Some(GD::default());

    match check_os_version(){
        Ok(_) => {}
        Err(e) => {
            println!("{}",e);
            __GD.take();
            return STATUS_UNSUCCESSFUL;
        }
    }

    match check_vmx_cpu_support() {
        Ok(_) => {}
        Err(e) => {
            println!("{}",e);
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

    driver_object.DriverUnload = Some(driver_unload);
    
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
                        println!("{}",e);
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
            println!("{}", err);
            __GD.take();
            return STATUS_UNSUCCESSFUL;
        }
    }

    // set dispatch function
    for i in 0..IRP_MJ_MAXIMUM_FUNCTION {
        driver_object.MajorFunction[i as usize] = Some(dispatch_device);
    }

    status
}

pub unsafe extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
    // clear resources when drvier unload
    __GD.take();

    println!("DriverUnload Success");
}