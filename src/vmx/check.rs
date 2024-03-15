extern crate alloc;

use core::arch::x86_64::__cpuid;

use moon_instructions::{read_msr, write_msr};
use moon_struct::msr::{self, ia32_feature_control_msr, ia32_mtrr_def_type_msr};
use wdk::println;

pub fn check_vmx_cpu_support() -> Result<(), &'static str> {
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

    // CPUID.1:ECX.VMX[bit 5] = 1
    if cpuid_result.ecx & (1 << 5) == 0 {
        return Err("CPU dont support vt");
    }

    // check bios switch
    let mut feature_control_msr = read_msr(msr::msr_index::MSR_IA32_FEATURE_CONTROL);

    if (feature_control_msr & ia32_feature_control_msr::LOCK_MASK) == 0 {
        feature_control_msr |= ia32_feature_control_msr::LOCK_MASK;
        feature_control_msr |= ia32_feature_control_msr::ENABLE_VMXON;
        write_msr(msr::msr_index::MSR_IA32_FEATURE_CONTROL, feature_control_msr);
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
    let mtrr_def_type_msr = read_msr(msr::msr_index::MSR_IA32_MTRR_DEF_TYPE);

    if (mtrr_def_type_msr & ia32_mtrr_def_type_msr::MTRR_ENABLE_MASK) == 0 {
        return Err("Mtrr dynamic ranges not supported");
    }

    println!("VMX cpu check success");

    return Ok(());
}
