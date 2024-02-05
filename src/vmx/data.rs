

pub mod vmx_cpu_based_controls {
    // pub(crate) const INVLPG_EXITING: u64 = 1 << 9;
    // pub(crate) const CR3_LOAD_EXITING: u64 = 1 << 15;
    // pub(crate) const CR3_STORE_EXITING: u64 = 1 << 15;
    pub(crate) const USE_MSR_BITMAPS: u64 = 1 << 28;
    pub(crate) const ACTIVATE_SECONDARY_CONTROL: u64 = 1 << 31;
}

pub mod vmx_secondary_cpu_based_controls {
    pub(crate) const ENABLE_RDTSCP: u64 = 1 << 3;
    pub(crate) const ENABLE_INVPCID: u64 = 1 << 12;
    pub(crate) const ENABLE_XSAVESX_STORS: u64 = 1 << 20;
}

pub mod vmx_vm_enter_controls {
    pub(crate) const LOAD_DEBUG_CONTROLS: u64 = 1 << 2;
    pub(crate) const IA32E_MODE_GUEST: u64 = 1 << 9;
}

pub mod vmx_vm_exit_controls {
    pub(crate) const HOST_ADDRESS_SPACE_SIZE: u64 = 1 << 9;
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
