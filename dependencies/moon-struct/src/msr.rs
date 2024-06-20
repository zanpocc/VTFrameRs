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
    /** Tertiary processor-based VM execution controls. */
    pub const MSR_IA32_VMX_PROCBASED_CTLS3: u32 = 0x492;
    /** Secondary VM-exit controls. */
    pub const MSR_IA32_VMX_EXIT_CTLS2: u32 = 0x493;

    pub const MSR_IA32_SYSENTER_CS: u32 = 0x174;
    pub const MSR_IA32_SYSENTER_ESP: u32 = 0x175;
    pub const MSR_IA32_SYSENTER_EIP: u32 = 0x176;
    pub const MSR_IA32_DEBUGCTL: u32 = 0x1D9;
    pub const MSR_LSTAR: u32 = 0xC0000082;
    pub const MSR_FS_BASE: u32 = 0xC0000100;
    pub const MSR_GS_BASE: u32 = 0xC0000101;
    pub const MSR_SHADOW_GS_BASE: u32 = 0xC0000102; // SwapGS GS shadow

    pub const MSR_IA32_MTRR_DEF_TYPE: u32 = 0x000002FF;

    // MTRR Capabilities MSR
    pub const MSR_IA32_MTRR_CAPABILITIES: u32 = 0x000000FE;

    // MTRR Physical Base MSRs
    pub const MSR_IA32_MTRR_PHYSBASE0: u32 = 0x00000200;
    pub const MSR_IA32_MTRR_PHYSBASE1: u32 = 0x00000202;
    pub const MSR_IA32_MTRR_PHYSBASE2: u32 = 0x00000204;
    pub const MSR_IA32_MTRR_PHYSBASE3: u32 = 0x00000206;
    pub const MSR_IA32_MTRR_PHYSBASE4: u32 = 0x00000208;
    pub const MSR_IA32_MTRR_PHYSBASE5: u32 = 0x0000020A;
    pub const MSR_IA32_MTRR_PHYSBASE6: u32 = 0x0000020C;
    pub const MSR_IA32_MTRR_PHYSBASE7: u32 = 0x0000020E;
    pub const MSR_IA32_MTRR_PHYSBASE8: u32 = 0x00000210;
    pub const MSR_IA32_MTRR_PHYSBASE9: u32 = 0x00000212;

    // MTRR Physical Mask MSRs
    pub const MSR_IA32_MTRR_PHYSMASK0: u32 = 0x00000201;
    pub const MSR_IA32_MTRR_PHYSMASK1: u32 = 0x00000203;
    pub const MSR_IA32_MTRR_PHYSMASK2: u32 = 0x00000205;
    pub const MSR_IA32_MTRR_PHYSMASK3: u32 = 0x00000207;
    pub const MSR_IA32_MTRR_PHYSMASK4: u32 = 0x00000209;
    pub const MSR_IA32_MTRR_PHYSMASK5: u32 = 0x0000020B;
    pub const MSR_IA32_MTRR_PHYSMASK6: u32 = 0x0000020D;
    pub const MSR_IA32_MTRR_PHYSMASK7: u32 = 0x0000020F;
    pub const MSR_IA32_MTRR_PHYSMASK8: u32 = 0x00000211;
    pub const MSR_IA32_MTRR_PHYSMASK9: u32 = 0x00000213;

    // vmware
    pub const MSR_STIMER0_CONFIG: u32 = 0x400000b0;
    pub const MSR_STIMER0_COUNT: u32 = 0x400000b1;

    pub const MSR_CRASH_CTL: u32 = 0x40000105;
    pub const MSR_CRASH_P0: u32 = 0x40000100;

    // RW will #GP
    pub const MSR_RESERVED_MIN: u32 = 0x40000000; // Reserved MSR Address Space Min
                                                  //pub const MSR_RESERVED_MAX: u32 =	0x400000FF; // Reserved MSR Address Space Max
    pub const MSR_RESERVED_MAX: u32 = 0xC0000079; // Reserved MSR Address Space Max

    pub const MSR_UNKNOWN: u32 = 0xc0002fff;
    pub const MSR_UNKNOWN2: u32 = 0x00002fff;
}

pub mod ia32_mtrr_capabilities_msr {
    pub const VARIABLE_RANGE_COUNT_START: u64 = 0;
    pub const VARIABLE_RANGE_COUNT_LEN: u64 = 8;

    pub const PAGE_FRAME_NUMBER_START: u64 = 12;
    pub const PAGE_FRAME_NUMBER_LEN: u64 = 36;
}

pub mod ia32_mtrr_phys_base_msr {

    pub const TYPE_START: u64 = 0;
    pub const TYPE_LEN: u64 = 8;

    pub const PAGE_FRAME_NUMBER_START: u64 = 12;
    pub const PAGE_FRAME_NUMBER_LEN: u64 = 36;
}

pub mod ia32_mtrr_phys_mask_msr {
    use crate::RT_BIT_64;

    pub const VALID: u64 = RT_BIT_64!(11);

    pub const PAGE_FRAME_NUMBER_START: u64 = 12;
    pub const PAGE_FRAME_NUMBER_LEN: u64 = 36;
}

pub mod ia32_feature_control_msr {
    use crate::RT_BIT_64;

    /** Feature control - Lock MSR from writes (R/W0). */
    pub const MSR_IA32_FEATURE_CONTROL_LOCK: u64 = RT_BIT_64!(0);
    /** Feature control - Enable VMX inside SMX operation (R/WL). */
    pub const MSR_IA32_FEATURE_CONTROL_SMX_VMXON: u64 = RT_BIT_64!(1);
    /** Feature control - Enable VMX outside SMX operation (R/WL). */
    pub const MSR_IA32_FEATURE_CONTROL_VMXON: u64 = RT_BIT_64!(2);
    /** Feature control - SENTER local functions enable (R/WL).  */
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_0: u64 = RT_BIT_64!(8);
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_1: u64 = RT_BIT_64!(9);
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_2: u64 = RT_BIT_64!(10);
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_3: u64 = RT_BIT_64!(11);
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_4: u64 = RT_BIT_64!(12);
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_5: u64 = RT_BIT_64!(13);
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_LOCAL_FN_6: u64 = RT_BIT_64!(14);
    /** Feature control - SENTER global enable (R/WL). */
    pub const MSR_IA32_FEATURE_CONTROL_SENTER_GLOBAL_EN: u64 = RT_BIT_64!(15);
    /** Feature control - SGX launch control enable (R/WL). */
    pub const MSR_IA32_FEATURE_CONTROL_SGX_LAUNCH_EN: u64 = RT_BIT_64!(17);
    /** Feature control - SGX global enable (R/WL). */
    pub const MSR_IA32_FEATURE_CONTROL_SGX_GLOBAL_EN: u64 = RT_BIT_64!(18);
    /** Feature control - LMCE on (R/WL). */
    pub const MSR_IA32_FEATURE_CONTROL_LMCE: u64 = RT_BIT_64!(20);
}

pub mod ia32_vmx_basic_msr {
    use crate::RT_BIT_64;

    pub const VMX_CAPABILITY_HINT_MASK: u64 = RT_BIT_64!(55);
}

pub mod ia32_mtrr_def_type_msr {
    use crate::RT_BIT_64;

    pub const MTRR_ENABLE_MASK: u64 = RT_BIT_64!(11);
}

/// VMX MSR - EPT/VPID capabilities.
pub mod ia32_vmx_ept_vpid_cap_msr {
    use crate::RT_BIT_64;

    /** Supports execute-only translations by EPT. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_RWX_X_ONLY: u64 = RT_BIT_64!(0);
    /** Supports page-walk length of 4. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_PAGE_WALK_LENGTH_4: u64 = RT_BIT_64!(6);
    /** Supports page-walk length of 5. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_PAGE_WALK_LENGTH_5: u64 = RT_BIT_64!(7);
    /** Supports EPT paging-structure memory type to be uncacheable. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_MEMTYPE_UC: u64 = RT_BIT_64!(8);
    /** Supports EPT paging structure memory type to be write-back. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_MEMTYPE_WB: u64 = RT_BIT_64!(14);
    /** Supports EPT PDE to map a 2 MB page. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_PDE_2M: u64 = RT_BIT_64!(16);
    /** Supports EPT PDPTE to map a 1 GB page. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_PDPTE_1G: u64 = RT_BIT_64!(17);
    /** Supports INVEPT instruction. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVEPT: u64 = RT_BIT_64!(20);
    /** Supports accessed and dirty flags for EPT. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_ACCESS_DIRTY: u64 = RT_BIT_64!(21);
    /** Supports advanced VM-exit info. for EPT violations. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_ADVEXITINFO_EPT_VIOLATION: u64 = RT_BIT_64!(22);
    /** Supports supervisor shadow-stack control. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_SUPER_SHW_STACK: u64 = RT_BIT_64!(23);
    /** Supports single-context INVEPT type. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVEPT_SINGLE_CONTEXT: u64 = RT_BIT_64!(25);
    /** Supports all-context INVEPT type. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVEPT_ALL_CONTEXTS: u64 = RT_BIT_64!(26);
    /** Supports INVVPID instruction. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVVPID: u64 = RT_BIT_64!(32);
    /** Supports individual-address INVVPID type. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVVPID_INDIV_ADDR: u64 = RT_BIT_64!(40);
    /** Supports single-context INVVPID type. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVVPID_SINGLE_CONTEXT: u64 = RT_BIT_64!(41);
    /** Supports all-context INVVPID type. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVVPID_ALL_CONTEXTS: u64 = RT_BIT_64!(42);
    /** Supports singe-context-retaining-globals INVVPID type. */
    pub const MSR_IA32_VMX_EPT_VPID_CAP_INVVPID_SINGLE_CONTEXT_RETAIN_GLOBALS: u64 = RT_BIT_64!(43);
}
