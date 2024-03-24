

/// VMX MSR - Basic VMX information.
pub mod vmx_basic {
    use moon_struct::RT_BIT_64;

    /** Whether 'true' VMX controls MSRs are supported for handling of default1 class
 *  bits in VMX control MSRs. */
    pub const VMX_BASIC_TRUE_CTLS:u64 = RT_BIT_64!(55);
}

/// Processor-based VM-execution controls.
pub mod vmx_cpu_based_controls {
    use moon_struct::RT_BIT_32;
    
    /** VM-exit as soon as RFLAGS.IF=1 and no blocking is active. */
    pub const VMX_PROC_CTLS_INT_WINDOW_EXIT:u32 = RT_BIT_32!(2);
    /** Use timestamp counter offset. */
    pub const VMX_PROC_CTLS_USE_TSC_OFFSETTING:u32 = RT_BIT_32!(3);
    /** VM-exit when executing the HLT instruction. */
    pub const VMX_PROC_CTLS_HLT_EXIT:u32 = RT_BIT_32!(7);
    /** VM-exit when executing the INVLPG instruction. */
    pub const VMX_PROC_CTLS_INVLPG_EXIT:u32 = RT_BIT_32!(9);
    /** VM-exit when executing the MWAIT instruction. */
    pub const VMX_PROC_CTLS_MWAIT_EXIT:u32 = RT_BIT_32!(10);
    /** VM-exit when executing the RDPMC instruction. */
    pub const VMX_PROC_CTLS_RDPMC_EXIT:u32 = RT_BIT_32!(11);
    /** VM-exit when executing the RDTSC/RDTSCP instruction. */
    pub const VMX_PROC_CTLS_RDTSC_EXIT:u32 = RT_BIT_32!(12);
    /** VM-exit when executing the MOV to CR3 instruction. (forced to 1 on the
     *  'first' VT-x capable CPUs; this actually includes the newest Nehalem CPUs) */
    pub const VMX_PROC_CTLS_CR3_LOAD_EXIT:u32 = RT_BIT_32!(15);
    /** VM-exit when executing the MOV from CR3 instruction. (forced to 1 on the
     *  'first' VT-x capable CPUs; this actually includes the newest Nehalem CPUs) */
    pub const VMX_PROC_CTLS_CR3_STORE_EXIT:u32 = RT_BIT_32!(16);
    /** Whether the secondary processor based VM-execution controls are used. */
    pub const VMX_PROC_CTLS_USE_TERTIARY_CTLS:u32 = RT_BIT_32!(17);
    /** VM-exit on CR8 loads. */
    pub const VMX_PROC_CTLS_CR8_LOAD_EXIT:u32 = RT_BIT_32!(19);
    /** VM-exit on CR8 stores. */
    pub const VMX_PROC_CTLS_CR8_STORE_EXIT:u32 = RT_BIT_32!(20);
    /** Use TPR shadow. */
    pub const VMX_PROC_CTLS_USE_TPR_SHADOW:u32 = RT_BIT_32!(21);
    /** VM-exit when virtual NMI blocking is disabled. */
    pub const VMX_PROC_CTLS_NMI_WINDOW_EXIT:u32 = RT_BIT_32!(22);
    /** VM-exit when executing a MOV DRx instruction. */
    pub const VMX_PROC_CTLS_MOV_DR_EXIT:u32 = RT_BIT_32!(23);
    /** VM-exit when executing IO instructions. */
    pub const VMX_PROC_CTLS_UNCOND_IO_EXIT:u32 = RT_BIT_32!(24);
    /** Use IO bitmaps. */
    pub const VMX_PROC_CTLS_USE_IO_BITMAPS:u32 = RT_BIT_32!(25);
    /** Monitor trap flag. */
    pub const VMX_PROC_CTLS_MONITOR_TRAP_FLAG:u32 = RT_BIT_32!(27);
    /** Use MSR bitmaps. */
    pub const VMX_PROC_CTLS_USE_MSR_BITMAPS:u32 = RT_BIT_32!(28);
    /** VM-exit when executing the MONITOR instruction. */
    pub const VMX_PROC_CTLS_MONITOR_EXIT:u32 = RT_BIT_32!(29);
    /** VM-exit when executing the PAUSE instruction. */
    pub const VMX_PROC_CTLS_PAUSE_EXIT:u32 = RT_BIT_32!(30);
    /** Whether the secondary processor based VM-execution controls are used. */
    pub const VMX_PROC_CTLS_USE_SECONDARY_CTLS:u32 = RT_BIT_32!(31);
}

/// Secondary Processor-based VM-execution controls.
pub mod vmx_secondary_cpu_based_controls {
    use moon_struct::RT_BIT_32;

    /** Virtualize APIC accesses. */
    pub const VMX_PROC_CTLS2_VIRT_APIC_ACCESS:u32 = RT_BIT_32!(0);
    /** EPT supported/enabled. */
    pub const VMX_PROC_CTLS2_EPT:u32 = RT_BIT_32!(1);
    /** Descriptor table instructions cause VM-exits. */
    pub const VMX_PROC_CTLS2_DESC_TABLE_EXIT:u32 = RT_BIT_32!(2);
    /** RDTSCP supported/enabled. */
    pub const VMX_PROC_CTLS2_RDTSCP:u32 = RT_BIT_32!(3);
    /** Virtualize x2APIC mode. */
    pub const VMX_PROC_CTLS2_VIRT_X2APIC_MODE:u32 = RT_BIT_32!(4);
    /** VPID supported/enabled. */
    pub const VMX_PROC_CTLS2_VPID:u32 = RT_BIT_32!(5);
    /** VM-exit when executing the WBINVD instruction. */
    pub const VMX_PROC_CTLS2_WBINVD_EXIT:u32 = RT_BIT_32!(6);
    /** Unrestricted guest execution. */
    pub const VMX_PROC_CTLS2_UNRESTRICTED_GUEST:u32 = RT_BIT_32!(7);
    /** APIC register virtualization. */
    pub const VMX_PROC_CTLS2_APIC_REG_VIRT:u32 = RT_BIT_32!(8);
    /** Virtual-interrupt delivery. */
    pub const VMX_PROC_CTLS2_VIRT_INT_DELIVERY:u32 = RT_BIT_32!(9);
    /** A specified number of pause loops cause a VM-exit. */
    pub const VMX_PROC_CTLS2_PAUSE_LOOP_EXIT:u32 = RT_BIT_32!(10);
    /** VM-exit when executing RDRAND instructions. */
    pub const VMX_PROC_CTLS2_RDRAND_EXIT:u32 = RT_BIT_32!(11);
    /** Enables INVPCID instructions. */
    pub const VMX_PROC_CTLS2_INVPCID:u32 = RT_BIT_32!(12);
    /** Enables VMFUNC instructions. */
    pub const VMX_PROC_CTLS2_VMFUNC:u32 = RT_BIT_32!(13);
    /** Enables VMCS shadowing. */
    pub const VMX_PROC_CTLS2_VMCS_SHADOWING:u32 = RT_BIT_32!(14);
    /** Enables ENCLS VM-exits. */
    pub const VMX_PROC_CTLS2_ENCLS_EXIT:u32 = RT_BIT_32!(15);
    /** VM-exit when executing RDSEED. */
    pub const VMX_PROC_CTLS2_RDSEED_EXIT:u32 = RT_BIT_32!(16);
    /** Enables page-modification logging. */
    pub const VMX_PROC_CTLS2_PML:u32 = RT_BIT_32!(17);
    /** Controls whether EPT-violations may cause \#VE instead of exits. */
    pub const VMX_PROC_CTLS2_EPT_XCPT_VE:u32 = RT_BIT_32!(18);
    /** Conceal VMX non-root operation from Intel processor trace (PT). */
    pub const VMX_PROC_CTLS2_CONCEAL_VMX_FROM_PT:u32 = RT_BIT_32!(19);
    /** Enables XSAVES/XRSTORS instructions. */
    pub const VMX_PROC_CTLS2_XSAVES_XRSTORS:u32 = RT_BIT_32!(20);
    /** Enables supervisor/user mode based EPT execute permission for linear
     *  addresses. */
    pub const VMX_PROC_CTLS2_MODE_BASED_EPT_PERM:u32 = RT_BIT_32!(22);
    /** Enables EPT write permissions to be specified at granularity of 128 bytes. */
    pub const VMX_PROC_CTLS2_SPP_EPT:u32 = RT_BIT_32!(23);
    /** Intel PT output addresses are treated as guest-physical addresses and
     *  translated using EPT. */
    pub const VMX_PROC_CTLS2_PT_EPT:u32 = RT_BIT_32!(24);
    /** Use TSC scaling. */
    pub const VMX_PROC_CTLS2_TSC_SCALING:u32 = RT_BIT_32!(25);
    /** Enables TPAUSE, UMONITOR and UMWAIT instructions. */
    pub const VMX_PROC_CTLS2_USER_WAIT_PAUSE:u32 = RT_BIT_32!(26);
    /** Enables consulting ENCLV-exiting bitmap when executing ENCLV. */
    pub const VMX_PROC_CTLS2_ENCLV_EXIT:u32 = RT_BIT_32!(28);
}

/// VM-entry controls.
pub mod vmx_vm_enter_controls {
    use moon_struct::RT_BIT_32;

    /** Load guest debug controls (dr7 & IA32_DEBUGCTL_MSR) (forced to 1 on the
     *  'first' VT-x capable CPUs; this actually includes the newest Nehalem CPUs) */
    pub const VMX_ENTRY_CTLS_LOAD_DEBUG:u32 = RT_BIT_32!(2);
    /** 64-bit guest mode. Must be 0 for CPUs that don't support AMD64. */
    pub const VMX_ENTRY_CTLS_IA32E_MODE_GUEST:u32 = RT_BIT_32!(9);
    /** In SMM mode after VM-entry. */
    pub const VMX_ENTRY_CTLS_ENTRY_TO_SMM:u32 = RT_BIT_32!(10);
    /** Disable dual treatment of SMI and SMM; must be zero for VM-entry outside of SMM. */
    pub const VMX_ENTRY_CTLS_DEACTIVATE_DUAL_MON:u32 = RT_BIT_32!(11);
    /** Whether the guest IA32_PERF_GLOBAL_CTRL MSR is loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_PERF_MSR:u32 = RT_BIT_32!(13);
    /** Whether the guest IA32_PAT MSR is loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_PAT_MSR:u32 = RT_BIT_32!(14);
    /** Whether the guest IA32_EFER MSR is loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_EFER_MSR:u32 = RT_BIT_32!(15);
    /** Whether the guest IA32_BNDCFGS MSR is loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_BNDCFGS_MSR:u32 = RT_BIT_32!(16);
    /** Whether to conceal VMX from Intel PT (Processor Trace). */
    pub const VMX_ENTRY_CTLS_CONCEAL_VMX_FROM_PT:u32 = RT_BIT_32!(17);
    /** Whether the guest IA32_RTIT MSR is loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_RTIT_CTL_MSR:u32 = RT_BIT_32!(18);
    /** Whether the guest CET-related MSRs and SPP are loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_CET_STATE:u32 = RT_BIT_32!(20);
    /** Whether the guest IA32_PKRS MSR is loaded on VM-entry. */
    pub const VMX_ENTRY_CTLS_LOAD_PKRS_MSR:u32 = RT_BIT_32!(22);
}

/// VM-exit controls.
pub mod vmx_vm_exit_controls {
    use moon_struct::RT_BIT_32;

    /** Save guest debug controls (dr7 & IA32_DEBUGCTL_MSR) (forced to 1 on the
     *  'first' VT-x capable CPUs; this actually includes the newest Nehalem CPUs) */
    pub const VMX_EXIT_CTLS_SAVE_DEBUG:u32 = RT_BIT_32!(2);
    /** Return to long mode after a VM-exit. */
    pub const VMX_EXIT_CTLS_HOST_ADDR_SPACE_SIZE:u32 = RT_BIT_32!(9);
    /** Whether the host IA32_PERF_GLOBAL_CTRL MSR is loaded on VM-exit. */
    pub const VMX_EXIT_CTLS_LOAD_PERF_MSR:u32 = RT_BIT_32!(12);
    /** Acknowledge external interrupts with the irq controller if one caused a VM-exit. */
    pub const VMX_EXIT_CTLS_ACK_EXT_INT:u32 = RT_BIT_32!(15);
    /** Whether the guest IA32_PAT MSR is saved on VM-exit. */
    pub const VMX_EXIT_CTLS_SAVE_PAT_MSR:u32 = RT_BIT_32!(18);
    /** Whether the host IA32_PAT MSR is loaded on VM-exit. */
    pub const VMX_EXIT_CTLS_LOAD_PAT_MSR:u32 = RT_BIT_32!(19);
    /** Whether the guest IA32_EFER MSR is saved on VM-exit. */
    pub const VMX_EXIT_CTLS_SAVE_EFER_MSR:u32 = RT_BIT_32!(20);
    /** Whether the host IA32_EFER MSR is loaded on VM-exit. */
    pub const VMX_EXIT_CTLS_LOAD_EFER_MSR:u32 = RT_BIT_32!(21);
    /** Whether the value of the VMX preemption timer is saved on every VM-exit. */
    pub const VMX_EXIT_CTLS_SAVE_PREEMPT_TIMER:u32 = RT_BIT_32!(22);
    /** Whether IA32_BNDCFGS MSR is cleared on VM-exit. */
    pub const VMX_EXIT_CTLS_CLEAR_BNDCFGS_MSR:u32 = RT_BIT_32!(23);
    /** Whether to conceal VMX from Intel PT. */
    pub const VMX_EXIT_CTLS_CONCEAL_VMX_FROM_PT:u32 = RT_BIT_32!(24);
    /** Whether IA32_RTIT_CTL MSR is cleared on VM-exit. */
    pub const VMX_EXIT_CTLS_CLEAR_RTIT_CTL_MSR:u32 = RT_BIT_32!(25);
    /** Whether CET-related MSRs and SPP are loaded on VM-exit. */
    pub const VMX_EXIT_CTLS_LOAD_CET_STATE:u32 = RT_BIT_32!(28);
    /** Whether the host IA32_PKRS MSR is loaded on VM-exit. */
    pub const VMX_EXIT_CTLS_LOAD_PKRS_MSR:u32 = RT_BIT_32!(29);
    /** Whether the host IA32_PERF_GLOBAL_CTRL MSR is saved on VM-exit. */
    pub const VMX_EXIT_CTLS_SAVE_PERF_MSR:u32 = RT_BIT_32!(30);
    /** Whether secondary VM-exit controls are used. */
    pub const VMX_EXIT_CTLS_USE_SECONDARY_CTLS:u32 = RT_BIT_32!(31);
}


pub mod pml4e {
    use moon_struct::RT_BIT_64;

    pub const READ_ACCESS: u64 = RT_BIT_64!(0);
    pub const WRITE_ACCESS: u64 = RT_BIT_64!(1);
    pub const EXECUTE_ACCESS: u64 = RT_BIT_64!(2);

    pub const PAGE_FRAME_NUMBER_START: u64 = 12;
    pub const PAGE_FRAME_NUMBER_LEN: u64 = 36;
}

// pdpte,可以映射1GB
pub mod pml3e {
    use moon_struct::RT_BIT_64;

    pub const READ_ACCESS: u64 = RT_BIT_64!(0);
    pub const WRITE_ACCESS: u64 = RT_BIT_64!(1);
    pub const EXECUTE_ACCESS: u64 = RT_BIT_64!(2);

    pub const PAGE_FRAME_NUMBER_START: u64 = 12;
    pub const PAGE_FRAME_NUMBER_LEN: u64 = 36;
}

// pde,可以映射2MB
pub mod pml2e_2mb {
    use moon_struct::RT_BIT_64;

    pub const READ_ACCESS: u64 = RT_BIT_64!(0);
    pub const WRITE_ACCESS: u64 = RT_BIT_64!(1);
    pub const EXECUTE_ACCESS: u64 = RT_BIT_64!(2);

    pub const MEMORY_TYPE_START: u64 = 3;
    pub const MEMORY_TYPE_LEN: u64 = 3;
    
    pub const LARGET_PAGE: u64 = RT_BIT_64!(7);

    pub const PAGE_FRAME_NUMBER_START: u64 = 21;
    pub const PAGE_FRAME_NUMBER_LEN: u64 = 27;
}

// pte
pub mod ptee {

}


pub mod ept_pointer {
    use moon_struct::RT_BIT_64;

    // EPT Paging structure memory type (0 for UC)
    pub const MEMORY_TYPE_START:u64 = 0;
    pub const MEMORY_TYPE_LEN:u64 = 3;

    // PageWalkLength
    pub const PAGE_WALK_LENGTH_START:u64 = 3;
    pub const PAGE_WALK_LENGTH_LEN:u64 = 3;

    // EnableAccessAndDirtyFlags
    pub const ENABLE_ACCESS_AND_DIRTY_FLAGS: u64 = RT_BIT_64!(6);

    // Physical address of the EPT PML4 table
    pub const PHYS_ADDR_START:u64 = 12;
    pub const PHYS_ADDR_LEN:u64 = 36;
}

#[allow(dead_code)]
pub(crate) mod vmcs_encoding {
    pub const VIRTUAL_PROCESSOR_ID: u64 = 0x00000000;  // 16-Bit Control Field
    pub const POSTED_INTERRUPT_NOTIFICATION: u64 = 0x00000002;
    pub const EPTP_INDEX: u64 = 0x00000004;
    pub const GUEST_ES_SELECTOR: u64 = 0x00000800;  // 16-Bit Guest-State Fields
    pub const GUEST_CS_SELECTOR: u64 = 0x00000802;
    pub const GUEST_SS_SELECTOR: u64 = 0x00000804;
    pub const GUEST_DS_SELECTOR: u64 = 0x00000806;
    pub const GUEST_FS_SELECTOR: u64 = 0x00000808;
    pub const GUEST_GS_SELECTOR: u64 = 0x0000080a;
    pub const GUEST_LDTR_SELECTOR: u64 = 0x0000080c;
    pub const GUEST_TR_SELECTOR: u64 = 0x0000080e;
    pub const GUEST_INTERRUPT_STATUS: u64 = 0x00000810;
    pub const HOST_ES_SELECTOR: u64 = 0x00000c00;  // 16-Bit Host-State Fields
    pub const HOST_CS_SELECTOR: u64 = 0x00000c02;
    pub const HOST_SS_SELECTOR: u64 = 0x00000c04;
    pub const HOST_DS_SELECTOR: u64 = 0x00000c06;
    pub const HOST_FS_SELECTOR: u64 = 0x00000c08;
    pub const HOST_GS_SELECTOR: u64 = 0x00000c0a;
    pub const HOST_TR_SELECTOR: u64 = 0x00000c0c;
    pub const IO_BITMAP_A: u64 = 0x00002000;  // 64-Bit Control Fields
    pub const IO_BITMAP_A_HIGH: u64 = 0x00002001;
    pub const IO_BITMAP_B: u64 = 0x00002002;
    pub const IO_BITMAP_B_HIGH: u64 = 0x00002003;
    pub const MSR_BITMAP: u64 = 0x00002004;
    pub const MSR_BITMAP_HIGH: u64 = 0x00002005;
    pub const VM_EXIT_MSR_STORE_ADDR: u64 = 0x00002006;
    pub const VM_EXIT_MSR_STORE_ADDR_HIGH: u64 = 0x00002007;
    pub const VM_EXIT_MSR_LOAD_ADDR: u64 = 0x00002008;
    pub const VM_EXIT_MSR_LOAD_ADDR_HIGH: u64 = 0x00002009;
    pub const VM_ENTRY_MSR_LOAD_ADDR: u64 = 0x0000200a;
    pub const VM_ENTRY_MSR_LOAD_ADDR_HIGH: u64 = 0x0000200b;
    pub const EXECUTIVE_VMCS_POINTER: u64 = 0x0000200c;
    pub const EXECUTIVE_VMCS_POINTER_HIGH: u64 = 0x0000200d;
    pub const TSC_OFFSET: u64 = 0x00002010;
    pub const TSC_OFFSET_HIGH: u64 = 0x00002011;
    pub const VIRTUAL_APIC_PAGE_ADDR: u64 = 0x00002012;
    pub const VIRTUAL_APIC_PAGE_ADDR_HIGH: u64 = 0x00002013;
    pub const APIC_ACCESS_ADDR: u64 = 0x00002014;
    pub const APIC_ACCESS_ADDR_HIGH: u64 = 0x00002015;
    
    pub const EPT_POINTER: u64 = 0x0000201a;
    pub const EPT_POINTER_HIGH: u64 = 0x0000201b;
    pub const EOI_EXIT_BITMAP_0: u64 = 0x0000201c;
    pub const EOI_EXIT_BITMAP_0_HIGH: u64 = 0x0000201d;
    pub const EOI_EXIT_BITMAP_1: u64 = 0x0000201e;
    pub const EOI_EXIT_BITMAP_1_HIGH: u64 = 0x0000201f;
    pub const EOI_EXIT_BITMAP_2: u64 = 0x00002020;
    pub const EOI_EXIT_BITMAP_2_HIGH: u64 = 0x00002021;
    pub const EOI_EXIT_BITMAP_3: u64 = 0x00002022;
    pub const EOI_EXIT_BITMAP_3_HIGH: u64 = 0x00002023;
    pub const EPTP_LIST_ADDRESS: u64 = 0x00002024;
    pub const EPTP_LIST_ADDRESS_HIGH: u64 = 0x00002025;
    pub const VMREAD_BITMAP_ADDRESS: u64 = 0x00002026;
    pub const VMREAD_BITMAP_ADDRESS_HIGH: u64 = 0x00002027;
    pub const VMWRITE_BITMAP_ADDRESS: u64 = 0x00002028;
    pub const VMWRITE_BITMAP_ADDRESS_HIGH: u64 = 0x00002029;
    pub const VIRTUALIZATION_EXCEPTION_INFO_ADDDRESS: u64 = 0x0000202a;
    pub const VIRTUALIZATION_EXCEPTION_INFO_ADDDRESS_HIGH: u64 = 0x0000202b;
    pub const XSS_EXITING_BITMAP: u64 = 0x0000202c;
    pub const XSS_EXITING_BITMAP_HIGH: u64 = 0x0000202d;
    pub const GUEST_PHYSICAL_ADDRESS: u64 = 0x00002400;  // 64-Bit Read-Only Data Field
    pub const GUEST_PHYSICAL_ADDRESS_HIGH: u64 = 0x00002401;
    pub const VMCS_LINK_POINTER: u64 = 0x00002800;  // 64-Bit Guest-State Fields
    pub const VMCS_LINK_POINTER_HIGH: u64 = 0x00002801;
    pub const GUEST_IA32_DEBUGCTL: u64 = 0x00002802;
    pub const GUEST_IA32_DEBUGCTL_HIGH: u64 = 0x00002803;

    pub const GUEST_IA32_PAT: u64 = 0x00002804;
    pub const GUEST_IA32_PAT_HIGH: u64 = 0x00002805;
    pub const GUEST_IA32_EFER: u64 = 0x00002806;
    pub const GUEST_IA32_EFER_HIGH: u64 = 0x00002807;
    pub const GUEST_IA32_PERF_GLOBAL_CTRL: u64 = 0x00002808;
    pub const GUEST_IA32_PERF_GLOBAL_CTRL_HIGH: u64 = 0x00002809;
    pub const GUEST_PDPTR0: u64 = 0x0000280a;
    pub const GUEST_PDPTR0_HIGH: u64 = 0x0000280b;
    pub const GUEST_PDPTR1: u64 = 0x0000280c;
    pub const GUEST_PDPTR1_HIGH: u64 = 0x0000280d;
    pub const GUEST_PDPTR2: u64 = 0x0000280e;
    pub const GUEST_PDPTR2_HIGH: u64 = 0x0000280f;
    pub const GUEST_PDPTR3: u64 = 0x00002810;
    pub const GUEST_PDPTR3_HIGH: u64 = 0x00002811;
    pub const HOST_IA32_PAT: u64 = 0x00002c00;  // 64-Bit Host-State Fields
    pub const HOST_IA32_PAT_HIGH: u64 = 0x00002c01;
    pub const HOST_IA32_EFER: u64 = 0x00002c02;
    pub const HOST_IA32_EFER_HIGH: u64 = 0x00002c03;
    pub const HOST_IA32_PERF_GLOBAL_CTRL: u64 = 0x00002c04;
    pub const HOST_IA32_PERF_GLOBAL_CTRL_HIGH: u64 = 0x00002c05;
    pub const PIN_BASED_VM_EXEC_CONTROL: u64 = 0x00004000;  // 32-Bit Control Fields
    pub const CPU_BASED_VM_EXEC_CONTROL: u64 = 0x00004002;

    pub const EXCEPTION_BITMAP: u64 = 0x00004004;
    pub const PAGE_FAULT_ERROR_CODE_MASK: u64 = 0x00004006;
    pub const PAGE_FAULT_ERROR_CODE_MATCH: u64 = 0x00004008;
    pub const CR3_TARGET_COUNT: u64 = 0x0000400a;
    pub const VM_EXIT_CONTROLS: u64 = 0x0000400c;
    pub const VM_EXIT_MSR_STORE_COUNT: u64 = 0x0000400e;
    pub const VM_EXIT_MSR_LOAD_COUNT: u64 = 0x00004010;
    pub const VM_ENTRY_CONTROLS: u64 = 0x00004012;
    pub const VM_ENTRY_MSR_LOAD_COUNT: u64 = 0x00004014;
    pub const VM_ENTRY_INTR_INFO_FIELD: u64 = 0x00004016;
    pub const VM_ENTRY_EXCEPTION_ERROR_CODE: u64 = 0x00004018;
    pub const VM_ENTRY_INSTRUCTION_LEN: u64 = 0x0000401a;
    pub const TPR_THRESHOLD: u64 = 0x0000401c;
    pub const SECONDARY_VM_EXEC_CONTROL: u64 = 0x0000401e;
    pub const PLE_GAP: u64 = 0x00004020;
    pub const PLE_WINDOW: u64 = 0x00004022;
    pub const VM_INSTRUCTION_ERROR: u64 = 0x00004400;  // 32-Bit Read-Only Data Fields
    pub const VM_EXIT_REASON: u64 = 0x00004402;
    pub const VM_EXIT_INTR_INFO: u64 = 0x00004404;
    pub const VM_EXIT_INTR_ERROR_CODE: u64 = 0x00004406;
    pub const IDT_VECTORING_INFO_FIELD: u64 = 0x00004408;
    pub const IDT_VECTORING_ERROR_CODE: u64 = 0x0000440a;
    pub const VM_EXIT_INSTRUCTION_LEN: u64 = 0x0000440c;
    pub const VMX_INSTRUCTION_INFO: u64 = 0x0000440e;


    pub const GUEST_ES_LIMIT: u64 = 0x00004800;  // 32-Bit Guest-State Fields
    pub const GUEST_CS_LIMIT: u64 = 0x00004802;
    pub const GUEST_SS_LIMIT: u64 = 0x00004804;
    pub const GUEST_DS_LIMIT: u64 = 0x00004806;
    pub const GUEST_FS_LIMIT: u64 = 0x00004808;
    pub const GUEST_GS_LIMIT: u64 = 0x0000480a;
    pub const GUEST_LDTR_LIMIT: u64 = 0x0000480c;
    pub const GUEST_TR_LIMIT: u64 = 0x0000480e;
    pub const GUEST_GDTR_LIMIT: u64 = 0x00004810;
    pub const GUEST_IDTR_LIMIT: u64 = 0x00004812;
    pub const GUEST_ES_AR_BYTES: u64 = 0x00004814;
    pub const GUEST_CS_AR_BYTES: u64 = 0x00004816;
    pub const GUEST_SS_AR_BYTES: u64 = 0x00004818;
    pub const GUEST_DS_AR_BYTES: u64 = 0x0000481a;
    pub const GUEST_FS_AR_BYTES: u64 = 0x0000481c;
    pub const GUEST_GS_AR_BYTES: u64 = 0x0000481e;
    pub const GUEST_LDTR_AR_BYTES: u64 = 0x00004820;
    pub const GUEST_TR_AR_BYTES: u64 = 0x00004822;
    pub const GUEST_INTERRUPTIBILITY_INFO: u64 = 0x00004824;
    pub const GUEST_ACTIVITY_STATE: u64 = 0x00004826;
    pub const GUEST_SMBASE: u64 = 0x00004828;
    pub const GUEST_SYSENTER_CS: u64 = 0x0000482a;
    pub const VMX_PREEMPTION_TIMER_VALUE: u64 = 0x0000482e;
    pub const HOST_IA32_SYSENTER_CS: u64 = 0x00004c00;  // 32-Bit Host-State Field
    pub const CR0_GUEST_HOST_MASK: u64 = 0x00006000;    // Natural-Width Control Fields
    pub const CR4_GUEST_HOST_MASK: u64 = 0x00006002;
    pub const CR0_READ_SHADOW: u64 = 0x00006004;
    pub const CR4_READ_SHADOW: u64 = 0x00006006;
    pub const CR3_TARGET_VALUE0: u64 = 0x00006008;
    pub const CR3_TARGET_VALUE1: u64 = 0x0000600a;
    pub const CR3_TARGET_VALUE2: u64 = 0x0000600c;
    pub const CR3_TARGET_VALUE3: u64 = 0x0000600e;

    pub const EXIT_QUALIFICATION: u64 = 0x00006400;  // Natural-Width Read-Only Data Fields
    pub const IO_RCX: u64 = 0x00006402;
    pub const IO_RSI: u64 = 0x00006404;
    pub const IO_RDI: u64 = 0x00006406;
    pub const IO_RIP: u64 = 0x00006408;
    pub const GUEST_LINEAR_ADDRESS: u64 = 0x0000640a;
    pub const GUEST_CR0: u64 = 0x00006800;  // Natural-Width Guest-State Fields
    pub const GUEST_CR3: u64 = 0x00006802;
    pub const GUEST_CR4: u64 = 0x00006804;
    pub const GUEST_ES_BASE: u64 = 0x00006806;
    pub const GUEST_CS_BASE: u64 = 0x00006808;
    pub const GUEST_SS_BASE: u64 = 0x0000680a;
    pub const GUEST_DS_BASE: u64 = 0x0000680c;
    pub const GUEST_FS_BASE: u64 = 0x0000680e;
    pub const GUEST_GS_BASE: u64 = 0x00006810;
    pub const GUEST_LDTR_BASE: u64 = 0x00006812;
    pub const GUEST_TR_BASE: u64 = 0x00006814;
    pub const GUEST_GDTR_BASE: u64 = 0x00006816;
    pub const GUEST_IDTR_BASE: u64 = 0x00006818;
    pub const GUEST_DR7: u64 = 0x0000681a;
    pub const GUEST_RSP: u64 = 0x0000681c;
    pub const GUEST_RIP: u64 = 0x0000681e;
    pub const GUEST_RFLAGS: u64 = 0x00006820;
    pub const GUEST_PENDING_DBG_EXCEPTIONS: u64 = 0x00006822;
    pub const GUEST_SYSENTER_ESP: u64 = 0x00006824;
    pub const GUEST_SYSENTER_EIP: u64 = 0x00006826;
    pub const HOST_CR0: u64 = 0x00006c00;  // Natural-Width Host-State Fields
    pub const HOST_CR3: u64 = 0x00006c02;
    pub const HOST_CR4: u64 = 0x00006c04;
    pub const HOST_FS_BASE: u64 = 0x00006c06;
    pub const HOST_GS_BASE: u64 = 0x00006c08;
    pub const HOST_TR_BASE: u64 = 0x00006c0a;
    pub const HOST_GDTR_BASE: u64 = 0x00006c0c;
    pub const HOST_IDTR_BASE: u64 = 0x00006c0e;
    pub const HOST_IA32_SYSENTER_ESP: u64 = 0x00006c10;
    pub const HOST_IA32_SYSENTER_EIP: u64 = 0x00006c12;
    pub const HOST_RSP: u64 = 0x00006c14;
    pub const HOST_RIP: u64 = 0x00006c16;
}

#[allow(dead_code)]
pub(crate) mod exit_reason{
    pub const EXIT_REASON_EXCEPTION_NMI: u16 = 0;
    pub const EXIT_REASON_EXTERNAL_INTERRUPT: u16 = 1;
    pub const EXIT_REASON_TRIPLE_FAULT: u16 = 2;
    pub const EXIT_REASON_INIT: u16 = 3;
    pub const EXIT_REASON_SIPI: u16 = 4;
    pub const EXIT_REASON_IO_SMI: u16 = 5;
    pub const EXIT_REASON_OTHER_SMI: u16 = 6;
    pub const EXIT_REASON_PENDING_INTERRUPT: u16 = 7;
    pub const EXIT_REASON_NMI_WINDOW: u16 = 8;
    pub const EXIT_REASON_TASK_SWITCH: u16 = 9;
    pub const EXIT_REASON_CPUID: u16 = 10;
    pub const EXIT_REASON_GETSEC: u16 = 11;
    pub const EXIT_REASON_HLT: u16 = 12;
    pub const EXIT_REASON_INVD: u16 = 13;
    pub const EXIT_REASON_INVLPG: u16 = 14;
    pub const EXIT_REASON_RDPMC: u16 = 15;
    pub const EXIT_REASON_RDTSC: u16 = 16;
    pub const EXIT_REASON_RSM: u16 = 17;
    pub const EXIT_REASON_VMCALL: u16 = 18;
    pub const EXIT_REASON_VMCLEAR: u16 = 19;
    pub const EXIT_REASON_VMLAUNCH: u16 = 20;
    pub const EXIT_REASON_VMPTRLD: u16 = 21;
    pub const EXIT_REASON_VMPTRST: u16 = 22;
    pub const EXIT_REASON_VMREAD: u16 = 23;
    pub const EXIT_REASON_VMRESUME: u16 = 24;
    pub const EXIT_REASON_VMWRITE: u16 = 25;
    pub const EXIT_REASON_VMXOFF: u16 = 26;
    pub const EXIT_REASON_VMXON: u16 = 27;
    pub const EXIT_REASON_CR_ACCESS: u16 = 28;
    pub const EXIT_REASON_DR_ACCESS: u16 = 29;
    pub const EXIT_REASON_IO_INSTRUCTION: u16 = 30;
    pub const EXIT_REASON_MSR_READ: u16 = 31;
    pub const EXIT_REASON_MSR_WRITE: u16 = 32;
    pub const EXIT_REASON_INVALID_GUEST_STATE: u16 = 33;
    pub const EXIT_REASON_MSR_LOADING: u16 = 34;
    pub const EXIT_REASON_RESERVED_35: u16 = 35;
    pub const EXIT_REASON_MWAIT_INSTRUCTION: u16 = 36;
    pub const EXIT_REASOM_MTF: u16 = 37;
    pub const EXIT_REASON_RESERVED_38: u16 = 38;
    pub const EXIT_REASON_MONITOR_INSTRUCTION: u16 = 39;
    pub const EXIT_REASON_PAUSE_INSTRUCTION: u16 = 40;
    pub const EXIT_REASON_MACHINE_CHECK: u16 = 41;
    pub const EXIT_REASON_RESERVED_42: u16 = 42;
    pub const EXIT_REASON_TPR_BELOW_THRESHOLD: u16 = 43;
    pub const EXIT_REASON_APIC_ACCESS: u16 = 44;
    pub const EXIT_REASON_VIRTUALIZED_EIO: u16 = 45;
    pub const EXIT_REASON_XDTR_ACCESS: u16 = 46;
    pub const EXIT_REASON_TR_ACCESS: u16 = 47;
    pub const EXIT_REASON_EPT_VIOLATION: u16 = 48;
    pub const EXIT_REASON_EPT_MISCONFIG: u16 = 49;
    pub const EXIT_REASON_INVEPT: u16 = 50;
    pub const EXIT_REASON_RDTSCP: u16 = 51;
    pub const EXIT_REASON_PREEMPT_TIMER: u16 = 52;
    pub const EXIT_REASON_INVVPID: u16 = 53;
    pub const EXIT_REASON_WBINVD: u16 = 54;
    pub const EXIT_REASON_XSETBV: u16 = 55;
    pub const EXIT_REASON_APIC_WRITE: u16 = 56;
    pub const EXIT_REASON_RDRAND: u16 = 57;
    pub const EXIT_REASON_INVPCID: u16 = 58;
    pub const EXIT_REASON_VMFUNC: u16 = 59;
    pub const EXIT_REASON_RESERVED_60: u16 = 60;
    pub const EXIT_REASON_RDSEED: u16 = 61;
    pub const EXIT_REASON_RESERVED_62: u16 = 62;
    pub const EXIT_REASON_XSAVES: u16 = 63;
    pub const EXIT_REASON_XRSTORS: u16 = 64;

    pub const VMX_MAX_GUEST_VMEXIT: u16 = 65;

}


pub const VM_INSTRUCTION_ERROR_MAP: [&str; 28] = [
    "Success",
    "VMCALL executed in VMX root operation",
    "VMCLEAR with invalid physical address",
    "VMCLEAR with VMXON pointer",
    "VMLAUNCH with non-clear VMCS",
    "VMRESUME with non-launched VMCS",
    "VMRESUME after VMXOFF (VMXOFF and VMXON between VMLAUNCH and VMRESUME)",
    "VM entry with invalid control field(s)",
    "VM entry with invalid host-state field(s)",
    "VMPTRLD with invalid physical address",
    "VMPTRLD with VMXON pointer",
    "VMPTRLD with incorrect VMCS revision identifier",
    "VMREAD/VMWRITE from/to unsupported VMCS component",
    "VMWRITE to read-only VMCS component",
    "VMXON executed in VMX root operation",
    "VM entry with invalid executive-VMCS pointer",
    "VM entry with non-launched executive VMCS",
    "VM entry with executive-VMCS pointer not VMXON pointer (when attempting to deactivate the dual-monitor treatment of SMIs and SMM)",
    "VMCALL with non-clear VMCS (when attempting to activate the dual-monitor treatment of SMIs and SMM)",
    "VMCALL with invalid VM-exit control fields",
    "Unknown", // 21
    "VMCALL with incorrect MSEG revision identifier (when attempting to activate the dual-monitor treatment of SMIs and SMM)",
    "VMXOFF under dual-monitor treatment of SMIs and SMM",
    "VMCALL with invalid SMM-monitor features (when attempting to activate the dual-monitor treatment of SMIs and SMM)",
    "VM entry with invalid VM-execution control fields in executive VMCS (when attempting to return from SMM)",
    "VM entry with events blocked by MOV SS",
    "Unknown", // 27
    "Invalid operand to INVEPT/INVVPID"
];

pub mod interrupt_type {
    pub const INTERRUPT_EXTERNAL: u32             = 0;
    pub const INTERRUPT_NMI: u32                  = 2;
    pub const INTERRUPT_HARDWARE_EXCEPTION: u32   = 3;
    pub const INTERRUPT_SOFTWARE: u32             = 4;
    pub const INTERRUPT_PRIVILIGED_EXCEPTION: u32 = 5;
    pub const INTERRUPT_SOFTWARE_EXCEPTION: u32   = 6;
    pub const INTERRUPT_OTHER_EVENT: u32          = 7;
}

pub mod vector_exception {
    pub const VECTOR_DIVIDE_ERROR_EXCEPTION:u8          = 0;
    pub const VECTOR_DEBUG_EXCEPTION:u8                 = 1;
    pub const VECTOR_NMI_INTERRUPT:u8                   = 2;
    pub const VECTOR_BREAKPOINT_EXCEPTION:u8            = 3;
    pub const VECTOR_OVERFLOW_EXCEPTION:u8              = 4;
    pub const VECTOR_BOUND_EXCEPTION:u8                 = 5;
    pub const VECTOR_INVALID_OPCODE_EXCEPTION:u8        = 6;
    pub const VECTOR_DEVICE_NOT_AVAILABLE_EXCEPTION:u8  = 7;
    pub const VECTOR_DOUBLE_FAULT_EXCEPTION:u8          = 8;
    pub const VECTOR_COPROCESSOR_SEGMENT_OVERRUN:u8     = 9;
    pub const VECTOR_INVALID_TSS_EXCEPTION:u8           = 10;
    pub const VECTOR_SEGMENT_NOT_PRESENT:u8             = 11;
    pub const VECTOR_STACK_FAULT_EXCEPTION:u8           = 12;
    pub const VECTOR_GENERAL_PROTECTION_EXCEPTION:u8    = 13;
    pub const VECTOR_PAGE_FAULT_EXCEPTION:u8            = 14;
    pub const VECTOR_X87_FLOATING_POINT_ERROR:u8        = 16;
    pub const VECTOR_ALIGNMENT_CHECK_EXCEPTION:u8       = 17;
    pub const VECTOR_MACHINE_CHECK_EXCEPTION:u8         = 18;
    pub const VECTOR_SIMD_FLOATING_POINT_EXCEPTION:u8   = 19;
    pub const VECTOR_VIRTUALIZATION_EXCEPTION:u8        = 20;
}

pub mod interrupt_inject_info {
    use moon_struct::RT_BIT_32;

    pub const VECTOR_START:u32 = 0;
    pub const VECTOR_LEN:u32 = 8;

    pub const TYPE_START:u32 = 8;
    pub const TYPE_LEN:u32 = 3;

    pub const DELIVER_ERROR_CODE:u32 = RT_BIT_32!(11);
    
    pub const VALID:u32 = RT_BIT_32!(31);
}

#[allow(unused)]
pub(crate) mod mov_cr_qualification {
    pub const CONTROL_REGISTER_MASK: u32 = 0x0000000F;
    pub const ACCESS_TYPE_MASK: u32 = 0x00000030;
    pub const LMSW_OPERAND_TYPE_MASK: u32 = 0x00000040;
    pub const RESERVED1_MASK: u32 = 0x00000080;
    pub const REGISTER_MASK: u32 = 0x00000F00;
    pub const RESERVED2_MASK: u32 = 0x0000F000;
    pub const LMSW_SOURCE_DATA_MASK: u32 = 0xFFFF0000;
}

pub mod ept_memory_type{
    // Memory Types
    pub const MEMORY_TYPE_UNCACHEABLE:u8     = 0x00000000;
    pub const MEMORY_TYPE_WRITE_COMBINING:u8 = 0x00000001;
    pub const MEMORY_TYPE_WRITE_THROUGH:u8   = 0x00000004;
    pub const MEMORY_TYPE_WRITE_PROTECTED:u8 = 0x00000005;
    pub const MEMORY_TYPE_WRITE_BACK:u8      = 0x00000006;
    pub const MEMORY_TYPE_INVALID:u8         = 0x000000FF;
}

//CR
pub const TYPE_CR_WRITE: u32 = 0;
pub const TYPE_CR_READ: u32 = 1;
pub const TYPE_CLTS: u32 = 2;
pub const TYPE_LMSW: u32 = 3;
//DR
pub const TYPE_DR_WRITE: u32 = 0;
pub const TYPE_DR_READ: u32 = 1;

pub(crate) mod vm_call{
    // close vt
    pub const VM_CALL_CLOSE_VT: u64 = 1;
}

