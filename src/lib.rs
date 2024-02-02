// 这是一个编译器属性，用于告诉编译器不链接标准库（std）
#![no_std]

extern crate alloc;

// 需要把用到的mod都加进来，不然编辑器好像会有问题
pub mod vmx;
pub mod cpu;
pub mod device;
pub mod driver;
pub mod utils;
pub mod gd;

// 当 Rust 编译器不处于测试模式时才编译该代码块
#[cfg(not(test))]
// panic处理
extern crate wdk_panic;

use device::device::Device;
use gd::gd::GD;
use wdk::println;
// 全局分配器
#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::{device::device::DeviceOperations, driver::driver::Driver, vmx::check::{check_os_version, check_vmx_cpu_support}};

static mut __GD:Option<GD> = Option::None;

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    let status = STATUS_SUCCESS;

    println!("Hello World");

    __GD = Some(GD::new());

    match check_os_version(){
        Ok(_v) => {}
        Err(e) => {
            println!("{}",e);
            return STATUS_UNSUCCESSFUL;
        }
    }

    match check_vmx_cpu_support() {
        Ok(_v) => {}
        Err(e) => {
            println!("{}",e);
            return STATUS_UNSUCCESSFUL;
        }
    }

    driver.DriverUnload = Some(driver_unload);
    
    
    let mut driver = Driver::from_raw(driver);

    struct Nothing{}
    impl DeviceOperations for Nothing{}
    match driver.create_device("\\Device\\20240202", 0x22, 0, 0, Nothing{}) {
        Ok(device) => {
            if let Some(gd) = __GD.as_mut() {
                gd.device = Some(device);
            }
        },
        Err(err) => {
            println!("{}", err);
            return STATUS_UNSUCCESSFUL;
        }
    } 
    status
}

pub unsafe extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
    println!("DriverUnload");
    __GD.take();
}
