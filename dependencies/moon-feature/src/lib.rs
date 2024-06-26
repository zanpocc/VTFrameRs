#![no_std]

use moon_driver_utils::registry::query_registry_string;
use moon_instructions::cpuidex;
use moon_struct::x86::{X86_CPUID_FEATURE_ECX_HVP, X86_CPUID_FEATURE_ECX_VMX};

#[derive(PartialEq, Eq)]
pub enum CpuManufacturer {
    AMD,
    INTEL,
    UNKNOWN,
}

pub fn vmx_support() -> bool {
    let cpuid_result = cpuidex(1, 0);

    if cpuid_result.ecx & X86_CPUID_FEATURE_ECX_VMX == 0 {
        return false;
    }

    true
}

pub fn hypervisor_present() -> bool {
    let cpuid_result = cpuidex(1, 0);

    if cpuid_result.ecx & X86_CPUID_FEATURE_ECX_HVP == 0 {
        return false;
    }

    true
}

pub fn cpu_manufacturer() -> CpuManufacturer {
    let cpuid_result = cpuidex(0, 0);

    let intel: [u8; 12] = *b"GenuineIntel";
    let amd: [u8; 12] = *b"AuthenticAMD";

    let mut result_string = [0u8; 12];
    result_string[..4].copy_from_slice(&cpuid_result.ebx.to_ne_bytes());
    result_string[4..8].copy_from_slice(&cpuid_result.edx.to_ne_bytes());
    result_string[8..12].copy_from_slice(&cpuid_result.ecx.to_ne_bytes());

    if result_string == intel {
        return CpuManufacturer::INTEL;
    }

    if result_string == amd {
        return CpuManufacturer::AMD;
    }

    CpuManufacturer::UNKNOWN
}

pub fn physical_address_width() -> u64 {
    let cpuid_result = cpuidex(0x80000008, 0);
    cpuid_result.eax as u64 & 0xffu64
}

pub fn virtual_address_width() -> u64 {
    let cpuid_result = cpuidex(0x80000008, 0);
    (cpuid_result.eax >> 8) as u64 & 0xffu64
}

pub fn in_vmware() -> bool {
    let v = query_registry_string(
        "\\Registry\\Machine\\Hardware\\Description\\System",
        "SystemBiosVersion",
    );

    if v.contains("VMware") {
        return true;
    }

    false
}
