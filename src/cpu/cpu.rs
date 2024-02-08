extern crate alloc;

pub mod ins {
    use core::arch::asm;

    use wdk::println;

    use super::stru::CPUID;

    pub fn segment_limit(selector: u64) -> u64 {
        let mut result: u64;

        unsafe {
            asm!(
                "xor rax,rax",
                "lsl rax, rcx",
                out("rax") result,
                in("rcx") selector,
                options(nostack, nomem)
            );
        }

        result
    }

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

    pub fn write_msr(msr: u32,value: u64) {
        unsafe {
            asm!(
                "rdmsr",
                in("ecx") msr,
                in("rax") value,
                options(nostack, nomem)
            );
        }
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

    pub fn read_cr3() -> u64 {
        let mut result: u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "mov rax,cr3",
                out("rax") result,
                options(nostack, nomem)
            );
        }

        if result == 0{
            println!("read_cr3 error");
        }

        result
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

    pub fn cpuidex(eax: i32,ecx:i32) -> CPUID {
        let mut result = CPUID::default();

        unsafe {
            asm!(
                "cpuid",
                "mov [rdi],eax",
                "mov [rdi+4],ebx",
                "mov [rdi+8],ecx",
                "mov [rdi+0xc],edx",
                in("ecx") ecx,
                in("eax") eax,
                in("rdi") &mut result,
                options(nostack, nomem)
            );
        }

        result
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

    pub mod eflags {
        pub const CF: u32 = 0x00000001;
        pub const RESERVED1: u32 = 0x00000002;
        pub const PF: u32 = 0x00000004;
        pub const RESERVED2: u32 = 0x00000008;
        pub const AF: u32 = 0x00000010;
        pub const RESERVED3: u32 = 0x00000020;
        pub const ZF: u32 = 0x00000040;
        pub const SF: u32 = 0x00000080;
        pub const TF: u32 = 0x00000100;
        pub const IF: u32 = 0x00000200;
        pub const DF: u32 = 0x00000400;
        pub const OF: u32 = 0x00000800;
        pub const IOPL: u32 = 0x00003000;
        pub const NT: u32 = 0x00004000;
        pub const RESERVED4: u32 = 0x00008000;
        pub const RF: u32 = 0x00010000;
        pub const VM: u32 = 0x00020000;
        pub const AC: u32 = 0x00040000;
        pub const VIF: u32 = 0x00080000;
        pub const VIP: u32 = 0x00100000;
        pub const ID: u32 = 0x00200000;
        pub const RESERVED5: u32 = 0xFFC00000;
    }

    #[derive(Default)]
    pub struct CPUID {
        pub eax: i32,
        pub ebx: i32,
        pub ecx: i32,
        pub edx: i32,
    }
}
