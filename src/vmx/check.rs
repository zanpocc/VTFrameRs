extern crate alloc;

use moon_feature::{cpu_manufacturer, vmx_support, CpuManufacturer};
use moon_instructions::{read_msr, write_msr};
use moon_struct::msr::{self, ia32_feature_control_msr, ia32_mtrr_def_type_msr};
use wdk::println;

pub fn check_vmx_cpu_support() -> Result<(), &'static str> {
    if cpu_manufacturer() != CpuManufacturer::INTEL {
        return Err("Only support intel cpu");
    }

    if !vmx_support() {
        return Err("CPU dont support vt");
    }

    // check bios switch
    let mut feature_control_msr = read_msr(msr::msr_index::MSR_IA32_FEATURE_CONTROL);

    // if lock bit reset,vmxon will gp
    // if lock bit set,rdmsr will gp
    if (feature_control_msr & ia32_feature_control_msr::MSR_IA32_FEATURE_CONTROL_LOCK) == 0 {
        feature_control_msr |= ia32_feature_control_msr::MSR_IA32_FEATURE_CONTROL_LOCK;
        feature_control_msr |= ia32_feature_control_msr::MSR_IA32_FEATURE_CONTROL_VMXON;
        write_msr(msr::msr_index::MSR_IA32_FEATURE_CONTROL, feature_control_msr);
        println!("Start set feature control MSR");
    } else if (feature_control_msr & ia32_feature_control_msr::MSR_IA32_FEATURE_CONTROL_VMXON) == 0 {
        // if vmxon reset,vmxon will gp outside smx operation
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
