#![no_std]

pub mod msr;
pub mod eflags;
pub mod cpuid;
pub mod inner;
pub mod m;
pub mod x86;

use wdk_sys::{ntddk::RtlGetVersion, NT_SUCCESS, RTL_OSVERSIONINFOW};

// todo
pub fn check_os_version() -> Result<(), &'static str> {
    // check system version
    let mut os_version = RTL_OSVERSIONINFOW {
        dwOSVersionInfoSize: 0,
        dwMajorVersion: 0,
        dwMinorVersion: 0,
        dwBuildNumber: 0,
        dwPlatformId: 0,
        szCSDVersion: [0; 128],
    };

    // transform to origin point
    let os_version_ptr: *mut RTL_OSVERSIONINFOW = &mut os_version;

    let status = unsafe { RtlGetVersion(os_version_ptr) };

    if !NT_SUCCESS(status) {
        return Err("Query Sstem Version Error");
    }
    
    return Ok(());
}