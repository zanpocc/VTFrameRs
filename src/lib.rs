// 这是一个编译器属性，用于告诉编译器不链接标准库（std）
#![no_std]

extern crate alloc;

// 当 Rust 编译器不处于测试模式时才编译该代码块
#[cfg(not(test))]
// panic处理
extern crate wdk_panic;

use wdk::println;
// 全局分配器
#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::{
   DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING
 };

 pub unsafe extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
   println!("DriverUnload");
 }
 
 #[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
 pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
 ) -> NTSTATUS {

   println!("Hello World");

   driver.DriverUnload = Some(driver_unload);
   0
 }