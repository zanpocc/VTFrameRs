extern crate alloc;

use moon_feature::{cpu_manufacturer, vmx_support, CpuManufacturer};
use moon_instructions::{read_msr, write_msr};
use moon_log::info;
use moon_struct::msr::{self, ia32_feature_control_msr, ia32_mtrr_def_type_msr};

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
        write_msr(
            msr::msr_index::MSR_IA32_FEATURE_CONTROL,
            feature_control_msr,
        );
        info!("Start set feature control MSR");
    } else if (feature_control_msr & ia32_feature_control_msr::MSR_IA32_FEATURE_CONTROL_VMXON) == 0
    {
        // if vmxon reset,vmxon will gp outside smx operation
        return Err("BIOS dont enable virtualazation");
    }

    // check mttr
    let mtrr_def_type_msr = read_msr(msr::msr_index::MSR_IA32_MTRR_DEF_TYPE);

    if (mtrr_def_type_msr & ia32_mtrr_def_type_msr::MTRR_ENABLE_MASK) == 0 {
        return Err("Mtrr dynamic ranges not supported");
    }

    info!("VMX cpu check success");

    Ok(())
}
