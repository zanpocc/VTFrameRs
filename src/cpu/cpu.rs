extern crate alloc;

pub mod ins {
    use core::arch::asm;

    pub fn read_msr(msr: u32) -> u64 {
        let mut result: u64;

        unsafe {
            asm!(
                "rdmsr",
                "shl rdx, 32",
                "add rax, rdx",
                out("rax") result,
                in("ecx") msr,
                options(nostack, nomem)
            );
        }
        result
    }

    // todo
    pub fn write_msr(msr: u32,value: u64) {
    }


    pub fn read_cr0() -> u64 {
        let mut result: u64;
        unsafe {
            asm!(
                "mov rax,cr0",
                out("rax") result,
                options(nostack, nomem)
            );
        }
        result
    }

    pub fn write_cr0(value: u64) {
        unsafe {
            asm!(
                "mov cr0,rcx",
                in("rcx") value,
                options(nostack, nomem)
            );
        }
    }

    pub fn read_cr4() -> u64 {
        let mut result: u64;
        unsafe {
            asm!(
                "mov rax,cr4",
                out("rax") result,
                options(nostack, nomem)
            );
        }
        result
    }

    pub fn write_cr4(value: u64) {
        unsafe {
            asm!(
                "mov cr4,rcx",
                in("rcx") value,
                options(nostack, nomem)
            );
        }
    }
}

pub mod stru {
    pub mod msr_index {
        // MSR Constants
        pub const MSR_APIC_BASE: u32 = 0x01B;
        pub const MSR_IA32_FEATURE_CONTROL: u32 = 0x03A;
        pub const MSR_IA32_VMX_BASIC: u32 = 0x480;
        pub const MSR_IA32_VMX_PINBASED_CTLS: u32 = 0x481;
        pub const MSR_IA32_VMX_PROCBASED_CTLS: u32 = 0x482;
        pub const MSR_IA32_VMX_EXIT_CTLS: u32 = 0x483;
        pub const MSR_IA32_VMX_ENTRY_CTLS: u32 = 0x484;
        pub const MSR_IA32_VMX_MISC: u32 = 0x485;
        pub const MSR_IA32_VMX_CR0_FIXED0: u32 = 0x486;
        pub const MSR_IA32_VMX_CR0_FIXED1: u32 = 0x487;
        pub const MSR_IA32_VMX_CR4_FIXED0: u32 = 0x488;
        pub const MSR_IA32_VMX_CR4_FIXED1: u32 = 0x489;
        pub const MSR_IA32_VMX_VMCS_ENUM: u32 = 0x48A;
        pub const MSR_IA32_VMX_PROCBASED_CTLS2: u32 = 0x48B;
        pub const MSR_IA32_VMX_EPT_VPID_CAP: u32 = 0x48C;
        pub const MSR_IA32_VMX_TRUE_PINBASED_CTLS: u32 = 0x48D;
        pub const MSR_IA32_VMX_TRUE_PROCBASED_CTLS: u32 = 0x48E;
        pub const MSR_IA32_VMX_TRUE_EXIT_CTLS: u32 = 0x48F;
        pub const MSR_IA32_VMX_TRUE_ENTRY_CTLS: u32 = 0x490;
        pub const MSR_IA32_VMX_VMFUNC: u32 = 0x491;
        pub const MSR_IA32_SYSENTER_CS: u32 = 0x174;
        pub const MSR_IA32_SYSENTER_ESP: u32 = 0x175;
        pub const MSR_IA32_SYSENTER_EIP: u32 = 0x176;
        pub const MSR_IA32_DEBUGCTL: u32 = 0x1D9;
        pub const MSR_LSTAR: u32 = 0xC0000082;
        pub const MSR_FS_BASE: u32 = 0xC0000100;
        pub const MSR_GS_BASE: u32 = 0xC0000101;
        pub const MSR_SHADOW_GS_BASE: u32 = 0xC0000102; // SwapGS GS shadow

        pub const MSR_IA32_MTRR_DEF_TYPE: u32 = 0x000002FF;
    }

    pub mod msr {

        pub mod ia32_feature_control_msr {
            pub const LOCK_MASK: u64 = 1 << 0;
            pub const ENABLE_VMXON: u64 = 1 << 2;
        }

        pub mod ia32_vmx_basic_msr {
            pub const VMX_CAPABILITY_HINT_MASK: u64 = 1 << 55;
        }

        pub mod ia32_mtrr_def_type_msr {
            pub const MTRR_ENABLE_MASK: u64 = 1 << 11;
        }
    }
}
