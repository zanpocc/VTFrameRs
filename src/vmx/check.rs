extern crate alloc;

use core::arch::x86_64::__cpuid;

use wdk::println;
use wdk_sys::{ntddk::RtlGetVersion, NT_SUCCESS, RTL_OSVERSIONINFOW};

use crate::cpu::cpu::{ins::read_msr, stru::{msr::{ia32_feature_control_msr, ia32_mtrr_def_type_msr}, msr_index::{MSR_IA32_FEATURE_CONTROL, MSR_IA32_MTRR_DEF_TYPE}}};

pub fn check_os_version() -> Result<bool, &'static str> {
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

    println!(
        "Check_os_version success:{},{},{},{}",
        os_version.dwBuildNumber,
        os_version.dwMajorVersion,
        os_version.dwMinorVersion,
        os_version.dwPlatformId
    );

    return Ok(true);
}

pub fn check_vmx_cpu_support() -> Result<bool, &'static str> {
    // check cpu type
    let cpuid_result = unsafe { __cpuid(0) };

    let intel_string: [u8; 12] = *b"GenuineIntel";
    let mut result_string = [0u8; 12];
    result_string[..4].copy_from_slice(&cpuid_result.ebx.to_ne_bytes());
    result_string[4..8].copy_from_slice(&cpuid_result.edx.to_ne_bytes());
    result_string[8..12].copy_from_slice(&cpuid_result.ecx.to_ne_bytes());

    if intel_string != result_string {
        return Err("Only support intel cpu");
    }

    // check vmx support
    let cpuid_result = unsafe { __cpuid(1) };

    if cpuid_result.ecx & (1 << 5) == 0 {
        return Err("CPU dont support vt");
    }

    // check bios switch
    let feature_control_msr = read_msr(MSR_IA32_FEATURE_CONTROL);

    if (feature_control_msr & ia32_feature_control_msr::LOCK_MASK) == 0 {
        // todo:写msr,lock、enablevmxon为true,分开做把，就不写在check方法里面了
        println!("Start set feature control MSR");
    } else if (feature_control_msr & ia32_feature_control_msr::ENABLE_VMXON) == 0 {
        return Err("BIOS dont enable virtualazation");
    }

    // check vmx true
    // let vmx_basic_msr = read_msr(MSR_IA32_VMX_BASIC);
    // println!("vmx_basic_msr:{:X}", vmx_basic_msr);

    // if (vmx_basic_msr & ia32_vmx_basic_msr::VMX_CAPABILITY_HINT_MASK) != 1 {
    //     println!("Dont suppor vmx true");
    //     return Err("Dont suppor vmx true");
    // }

    // check mttr
    let mtrr_def_type_msr = read_msr(MSR_IA32_MTRR_DEF_TYPE);

    if (mtrr_def_type_msr & ia32_mtrr_def_type_msr::MTRR_ENABLE_MASK) == 0 {
        return Err("Mtrr dynamic ranges not supported");
    }

    println!("VMX cpu check success");

    return Ok(true);
}
