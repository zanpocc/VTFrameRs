use core::{ffi::c_void, mem::size_of, ptr::null_mut};

use alloc::{boxed::Box, vec::Vec};
use moon_driver_utils::bitfield::{create_end_mask, get_bits_value, set_bits_value};
use moon_feature::in_vmware;
use moon_instructions::{read_msr, segment_limit, write_cr0, write_cr4};
use moon_log::{error, info};
use moon_struct::{
    inner::{GdtEntry64, GDTENTRY64_ACCESS_RIGHTS, KGDTENTRY64, KPROCESSOR_STATE},
    msr::{
        self, ia32_vmx_ept_vpid_cap_msr,
        msr_index::{
            MSR_IA32_VMX_BASIC, MSR_IA32_VMX_EPT_VPID_CAP, MSR_IA32_VMX_PROCBASED_CTLS,
            MSR_IA32_VMX_PROCBASED_CTLS2, MSR_IA32_VMX_TRUE_PROCBASED_CTLS,
        },
    },
};
use wdk_sys::{
    ntddk::{
        KeQueryActiveProcessorCount, KeRevertToUserAffinityThread, KeSetSystemAffinityThread,
        MmAllocateContiguousMemory, MmFreeContiguousMemory, MmGetPhysicalAddress,
        RtlCaptureContext, RtlInitializeBitMap, RtlSetBit,
    },
    KERNEL_STACK_SIZE, PAGE_READWRITE, PHYSICAL_ADDRESS, RTL_BITMAP, USHORT, _LARGE_INTEGER,
};

use crate::{
    inner::{KeSaveStateForHibernate, RtlRestoreContext},
    utils::{get_current_processor_idx, protect_non_paged_memory},
    vm::ins::{__vmx_read_error, __vmx_vmlaunch},
    __GD,
};

use super::{
    data::{
        vm_call::EXIT_VT,
        vmcs_encoding::{
            CPU_BASED_VM_EXEC_CONTROL, CR0_GUEST_HOST_MASK, CR0_READ_SHADOW, CR4_GUEST_HOST_MASK,
            CR4_READ_SHADOW, EPT_POINTER, GUEST_CR0, GUEST_CR3, GUEST_CR4, GUEST_CS_AR_BYTES,
            GUEST_CS_BASE, GUEST_CS_LIMIT, GUEST_CS_SELECTOR, GUEST_DR7, GUEST_DS_AR_BYTES,
            GUEST_DS_BASE, GUEST_DS_LIMIT, GUEST_DS_SELECTOR, GUEST_ES_AR_BYTES, GUEST_ES_BASE,
            GUEST_ES_LIMIT, GUEST_ES_SELECTOR, GUEST_FS_AR_BYTES, GUEST_FS_BASE, GUEST_FS_LIMIT,
            GUEST_FS_SELECTOR, GUEST_GDTR_BASE, GUEST_GDTR_LIMIT, GUEST_GS_AR_BYTES, GUEST_GS_BASE,
            GUEST_GS_LIMIT, GUEST_GS_SELECTOR, GUEST_IA32_DEBUGCTL, GUEST_IDTR_BASE,
            GUEST_IDTR_LIMIT, GUEST_LDTR_AR_BYTES, GUEST_LDTR_BASE, GUEST_LDTR_LIMIT,
            GUEST_LDTR_SELECTOR, GUEST_RFLAGS, GUEST_RIP, GUEST_RSP, GUEST_SS_AR_BYTES,
            GUEST_SS_BASE, GUEST_SS_LIMIT, GUEST_SS_SELECTOR, GUEST_TR_AR_BYTES, GUEST_TR_BASE,
            GUEST_TR_LIMIT, GUEST_TR_SELECTOR, HOST_CR0, HOST_CR3, HOST_CR4, HOST_CS_SELECTOR,
            HOST_DS_SELECTOR, HOST_ES_SELECTOR, HOST_FS_BASE, HOST_FS_SELECTOR, HOST_GDTR_BASE,
            HOST_GS_BASE, HOST_GS_SELECTOR, HOST_IDTR_BASE, HOST_RIP, HOST_RSP, HOST_SS_SELECTOR,
            HOST_TR_BASE, HOST_TR_SELECTOR, MSR_BITMAP, PIN_BASED_VM_EXEC_CONTROL,
            SECONDARY_VM_EXEC_CONTROL, VIRTUAL_PROCESSOR_ID, VMCS_LINK_POINTER, VM_ENTRY_CONTROLS,
            VM_EXIT_CONTROLS,
        },
        vmx_basic::VMX_BASIC_TRUE_CTLS,
        vmx_cpu_based_controls::{self, VMX_PROC_CTLS_USE_SECONDARY_CTLS},
        vmx_secondary_cpu_based_controls::{
            self, VMX_PROC_CTLS2_EPT, VMX_PROC_CTLS2_VMFUNC, VMX_PROC_CTLS2_VPID,
        },
        vmx_vm_enter_controls, vmx_vm_exit_controls,
    },
    ept::EptState,
    ins::{
        VmxInstructionResult, __vmx_off, __vmx_on, __vmx_vmcall, __vmx_vmclear, __vmx_vmptrld,
        __vmx_vmwrite,
    },
};

extern "C" {
    pub fn vmm_entry_point();
}

struct VmcsResources {
    vmxon: *mut VmxVmcs,
    vmcs: *mut VmxVmcs,
    vmm_stack: *mut c_void,
    msr_bitmap: *mut c_void,
}

pub struct Vcpu {
    cpu_index: usize,
    host_state: Box<KPROCESSOR_STATE>,
    vcpu_vmx_state: VcpuVmxState,
    vm_resources: VmcsResources,
    vmxon: bool,
}

pub struct Vmm {
    pub cpu_count: u32,
    pub vmx_features: VMXFeatures,
    pub ept_state: Option<EptState>,
    pub vcpu: Vec<Box<Vcpu>>,
}

pub struct StartVTError {}

impl Vcpu {
    // free vmm relate physical memory self
    pub fn free_physical_memory(&mut self) {
        let vmcs_resources = &mut self.vm_resources;
        let vmxon = &mut vmcs_resources.vmxon;
        let vmcs = &mut vmcs_resources.vmcs;
        let vmm_stack = &mut vmcs_resources.vmm_stack;
        let msr_bitmap = &mut vmcs_resources.msr_bitmap;

        if !vmxon.is_null() {
            unsafe {
                MmFreeContiguousMemory(core::mem::replace(vmxon, core::ptr::null_mut()) as _)
            };
        }
        if !vmcs.is_null() {
            unsafe { MmFreeContiguousMemory(core::mem::replace(vmcs, core::ptr::null_mut()) as _) };
        }
        if !vmm_stack.is_null() {
            unsafe {
                MmFreeContiguousMemory(core::mem::replace(vmm_stack, core::ptr::null_mut()) as _)
            };
        }
        if !msr_bitmap.is_null() {
            unsafe {
                MmFreeContiguousMemory(core::mem::replace(msr_bitmap, core::ptr::null_mut()) as _)
            };
        }
    }

    fn enter_vmx_root_mode(&mut self) -> Result<(), &'static str> {
        let vmx_basic = read_msr(msr::msr_index::MSR_IA32_VMX_BASIC);
        let cr0_fixed0 = read_msr(msr::msr_index::MSR_IA32_VMX_CR0_FIXED0);
        let cr0_fixed1 = read_msr(msr::msr_index::MSR_IA32_VMX_CR0_FIXED1);
        let cr4_fixed0 = read_msr(msr::msr_index::MSR_IA32_VMX_CR4_FIXED0);
        let cr4_fixed1 = read_msr(msr::msr_index::MSR_IA32_VMX_CR4_FIXED1);

        // revision_id
        let vmxon = unsafe { &mut *self.vm_resources.vmxon };
        let vmcs = unsafe { &mut *self.vm_resources.vmcs };
        vmxon.revision_id = (vmx_basic & create_end_mask(30)) as _; // 0-30
        vmcs.revision_id = (vmx_basic & create_end_mask(30)) as _; // 0-30

        // cr0 and cr4
        let host_state = &mut self.host_state;
        host_state.SpecialRegisters.Cr0 &= (cr0_fixed1 as u32) as u64; // lowpart
        host_state.SpecialRegisters.Cr0 |= (cr0_fixed0 as u32) as u64; // lowpart
        host_state.SpecialRegisters.Cr4 &= (cr4_fixed1 as u32) as u64; // lowpart
        host_state.SpecialRegisters.Cr4 |= (cr4_fixed0 as u32) as u64; // lowpart

        // update cr0 and cr4
        write_cr0(host_state.SpecialRegisters.Cr0);
        write_cr4(host_state.SpecialRegisters.Cr4);

        // vmxon
        let phys = unsafe {
            &mut MmGetPhysicalAddress(self.vm_resources.vmxon as *mut c_void) as *mut _LARGE_INTEGER
        };

        match __vmx_on(phys as _) {
            VmxInstructionResult::VmxSuccess => {}
            _ => {
                error!("vmx error code:{}", __vmx_read_error());
                return Err("vmxon execute fault");
            }
        }

        self.vmxon = true;

        // vmclear:unbind current cpu from vmcs
        let phys = unsafe {
            &mut MmGetPhysicalAddress(self.vm_resources.vmcs as *mut c_void) as *mut _LARGE_INTEGER
        };

        match __vmx_vmclear(phys as _) {
            VmxInstructionResult::VmxSuccess => {}
            _ => {
                error!("vmx error code:{}", __vmx_read_error());
                return Err("vmclear execute fault");
            }
        }

        // vmptrld:bind current cpu to vmcs
        match __vmx_vmptrld(phys as _) {
            VmxInstructionResult::VmxSuccess => {}
            _ => {
                error!("vmx error code:{}", __vmx_read_error());
                return Err("vmclear execute fault");
            }
        }

        Ok(())
    }

    fn vmxp_adjust_msr(&self, control_value: u64, desired_value: u32) -> u32 {
        let mut result = desired_value;
        result &= (control_value >> 32) as u32;
        result |= control_value as u32;
        result
    }

    fn convert_gdt_entry(&mut self, base: u64, selector: USHORT) -> GdtEntry64 {
        // limit
        let limit = segment_limit(selector as _);

        // transform raw point to struct refrence
        let gdt_entry: *mut KGDTENTRY64 = (base + (selector as u64 & !3)) as *mut _;
        let gdt_entry = unsafe { &mut *gdt_entry };

        // base
        let mut temp_base;

        temp_base = unsafe {
            (((gdt_entry.dummy.u.bytes.BaseHigh) as u64) << 24)
                | (((gdt_entry.dummy.u.bytes.BaseMiddle) as u64) << 16)
                | (gdt_entry.dummy.base_low) as u64 & u64::MAX
        };

        if (get_bits_value(unsafe { gdt_entry.dummy.u.bits } as _, 8, 5) & 0x10) == 0 {
            temp_base |= unsafe { (gdt_entry.dummy.base_upper as u64) << 32 }
        } else {
            temp_base |= 0;
        }

        // access right
        let mut access_rights: GDTENTRY64_ACCESS_RIGHTS = GDTENTRY64_ACCESS_RIGHTS {
            access_rights: 0u32,
        };

        unsafe { access_rights.bytes.flags1 = gdt_entry.dummy.u.bytes.Flags1 };
        unsafe { access_rights.bytes.flags2 = gdt_entry.dummy.u.bytes.Flags2 };

        // USHORT Reserved : 4;
        unsafe { access_rights.bits = set_bits_value(access_rights.bits as u64, 8, 4, 0) as u16 };

        // Unusable = !Present
        if (get_bits_value(unsafe { gdt_entry.dummy.u.bits as _ }, 15, 1)) == 0 {
            unsafe {
                access_rights.access_rights = set_bits_value(access_rights.bits as _, 16, 1, 1) as _
            };
        } else {
            unsafe {
                access_rights.access_rights = set_bits_value(access_rights.bits as _, 16, 1, 0) as _
            };
        }

        GdtEntry64 {
            selector,
            limit: limit as _,
            access_rights,
            base: temp_base,
        }
    }

    fn init_msr_bitmap(&mut self) {
        let bit_map_read_low: *mut u32 = self.vm_resources.msr_bitmap as _; // 0-00001FFF
        let bit_map_read_high: *mut u32 = (bit_map_read_low as u64 + 1024) as _; // C0000000 - C0001FFF
        let bit_map_write_low: *mut u32 = (bit_map_read_high as u64 + 1024) as _;
        let bit_map_write_high: *mut u32 = (bit_map_write_low as u64 + 1024) as _;

        let mut bit_map_read_low_header: RTL_BITMAP = RTL_BITMAP {
            SizeOfBitMap: 0,
            Buffer: core::ptr::null_mut(),
        };
        let mut bit_map_read_high_header: RTL_BITMAP = RTL_BITMAP {
            SizeOfBitMap: 0,
            Buffer: core::ptr::null_mut(),
        };
        let mut bit_map_write_low_header: RTL_BITMAP = RTL_BITMAP {
            SizeOfBitMap: 0,
            Buffer: core::ptr::null_mut(),
        };
        let mut bit_map_write_high_header: RTL_BITMAP = RTL_BITMAP {
            SizeOfBitMap: 0,
            Buffer: core::ptr::null_mut(),
        };

        unsafe {
            RtlInitializeBitMap(
                &mut bit_map_read_low_header as _,
                bit_map_read_low,
                1024 * 8,
            );
            RtlInitializeBitMap(
                &mut bit_map_read_high_header as _,
                bit_map_read_high,
                1024 * 8,
            );
            RtlInitializeBitMap(
                &mut bit_map_write_low_header as _,
                bit_map_write_low,
                1024 * 8,
            );
            RtlInitializeBitMap(
                &mut bit_map_write_high_header as _,
                bit_map_write_high,
                1024 * 8,
            );
        }

        // rw msr will vm-exit when specific msr bit set
        unsafe {
            RtlSetBit(
                &mut bit_map_read_low_header as _,
                msr::msr_index::MSR_IA32_FEATURE_CONTROL,
            );
            RtlSetBit(
                &mut bit_map_read_low_header as _,
                msr::msr_index::MSR_IA32_DEBUGCTL,
            );
            RtlSetBit(
                &mut bit_map_read_high_header as _,
                msr::msr_index::MSR_LSTAR - 0xC0000000,
            );

            RtlSetBit(
                &mut bit_map_write_low_header as _,
                msr::msr_index::MSR_IA32_FEATURE_CONTROL,
            );
            RtlSetBit(
                &mut bit_map_write_low_header as _,
                msr::msr_index::MSR_IA32_DEBUGCTL,
            );
            RtlSetBit(
                &mut bit_map_write_high_header as _,
                msr::msr_index::MSR_LSTAR - 0xC0000000,
            );
        }

        for i in msr::msr_index::MSR_IA32_VMX_BASIC..=msr::msr_index::MSR_IA32_VMX_VMFUNC {
            unsafe {
                RtlSetBit(&mut bit_map_read_low_header, i);
                RtlSetBit(&mut bit_map_write_low_header, i);
            }
        }

        // MSR BitMap
        __vmx_vmwrite(MSR_BITMAP, unsafe {
            MmGetPhysicalAddress(self.vm_resources.msr_bitmap).QuadPart as _
        });
    }

    fn set_vmcs_data(&mut self) {
        let vm_pin_ctl_requested: u32 = 0;
        let mut vm_cpu_ctl_requested: u32 = 0;

        let mut vm_enter_ctl_requested: u32 = 0;
        let mut vm_exit_ctl_requested: u32 = 0;

        let vmx_feature = unsafe { &__GD.as_mut().unwrap().vmm.as_mut().unwrap().vmx_features };

        // fixed bit
        let mut vmx_pin: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_PINBASED_CTLS);
        let mut vmx_cpu: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_PROCBASED_CTLS);

        // let vmx_tertiary: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_PROCBASED_CTLS3); // maybe need
        let mut vmx_exit: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_EXIT_CTLS);
        // let mut vmx_exit_secondary: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_EXIT_CTLS2); // maybe need
        let mut vmx_entry: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_ENTRY_CTLS);

        // true msr
        if vmx_feature.true_msrs {
            vmx_pin = read_msr(msr::msr_index::MSR_IA32_VMX_TRUE_PINBASED_CTLS);
            vmx_cpu = read_msr(msr::msr_index::MSR_IA32_VMX_TRUE_PROCBASED_CTLS);
            vmx_exit = read_msr(msr::msr_index::MSR_IA32_VMX_TRUE_EXIT_CTLS);
            vmx_entry = read_msr(msr::msr_index::MSR_IA32_VMX_TRUE_ENTRY_CTLS);
        }

        if vmx_feature.secondary_controls {
            vm_cpu_ctl_requested |= vmx_cpu_based_controls::VMX_PROC_CTLS_USE_SECONDARY_CTLS;

            let mut vm_cpu_ctl2_requested: u32 = 0;

            // ept
            if vmx_feature.ept {
                let vmx_data = unsafe { __GD.as_mut().unwrap().vmm.as_mut().unwrap() };
                vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::VMX_PROC_CTLS2_EPT;

                if vmx_feature.vpid {
                    vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::VMX_PROC_CTLS2_VPID;
                    __vmx_vmwrite(VIRTUAL_PROCESSOR_ID, 1); // greater than 0
                }

                __vmx_vmwrite(
                    EPT_POINTER,
                    vmx_data.ept_state.as_mut().unwrap().get_ept_pointer(),
                );
            }

            let vmx_cpu_secondary: u64 = read_msr(msr::msr_index::MSR_IA32_VMX_PROCBASED_CTLS2);

            // other
            vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::VMX_PROC_CTLS2_RDTSCP;
            vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::VMX_PROC_CTLS2_INVPCID;
            vm_cpu_ctl2_requested |=
                vmx_secondary_cpu_based_controls::VMX_PROC_CTLS2_XSAVES_XRSTORS;

            //Secondary
            __vmx_vmwrite(
                SECONDARY_VM_EXEC_CONTROL as _,
                self.vmxp_adjust_msr(vmx_cpu_secondary, vm_cpu_ctl2_requested) as u64,
            );
        }

        // cpu
        vm_cpu_ctl_requested |= vmx_cpu_based_controls::VMX_PROC_CTLS_USE_MSR_BITMAPS; // msr
        vm_cpu_ctl_requested |= vmx_cpu_based_controls::VMX_PROC_CTLS_USE_TSC_OFFSETTING; // combine with rdtscp

        // vm_enter
        vm_enter_ctl_requested |= vmx_vm_enter_controls::VMX_ENTRY_CTLS_LOAD_DEBUG; // dr
        vm_enter_ctl_requested |= vmx_vm_enter_controls::VMX_ENTRY_CTLS_IA32E_MODE_GUEST;

        // vm_exit
        vm_exit_ctl_requested |= vmx_vm_exit_controls::VMX_EXIT_CTLS_HOST_ADDR_SPACE_SIZE;

        // msr bitmap
        self.init_msr_bitmap();

        // non root mode execute vmread can get this value
        __vmx_vmwrite(VMCS_LINK_POINTER as _, u64::MAX);

        //PIN
        __vmx_vmwrite(
            PIN_BASED_VM_EXEC_CONTROL as _,
            self.vmxp_adjust_msr(vmx_pin, vm_pin_ctl_requested) as u64,
        );

        //CPU Processor
        __vmx_vmwrite(
            CPU_BASED_VM_EXEC_CONTROL as _,
            self.vmxp_adjust_msr(vmx_cpu, vm_cpu_ctl_requested) as u64,
        );

        //VM Exit
        __vmx_vmwrite(
            VM_EXIT_CONTROLS as _,
            self.vmxp_adjust_msr(vmx_exit, vm_exit_ctl_requested) as u64,
        );

        //VM Entry
        __vmx_vmwrite(
            VM_ENTRY_CONTROLS as _,
            self.vmxp_adjust_msr(vmx_entry, vm_enter_ctl_requested) as u64,
        );

        // cs
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.Context_frame.SegCs,
        );
        __vmx_vmwrite(GUEST_CS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_CS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_CS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_CS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(
            HOST_CS_SELECTOR,
            (self.host_state.Context_frame.SegCs & !3) as _,
        );

        // ds
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.Context_frame.SegDs,
        );
        __vmx_vmwrite(GUEST_DS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_DS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_DS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_DS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(
            HOST_DS_SELECTOR,
            (self.host_state.Context_frame.SegDs & !3) as _,
        );

        // es
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.Context_frame.SegEs,
        );
        __vmx_vmwrite(GUEST_ES_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_ES_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_ES_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_ES_BASE, gdt_entry.base as _);
        __vmx_vmwrite(
            HOST_ES_SELECTOR,
            (self.host_state.Context_frame.SegEs & !3) as _,
        );

        // fs
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.Context_frame.SegFs,
        );
        __vmx_vmwrite(GUEST_FS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_FS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_FS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_FS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_FS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(
            HOST_FS_SELECTOR,
            (self.host_state.Context_frame.SegFs & !3) as _,
        );

        // gs
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.Context_frame.SegGs,
        );
        __vmx_vmwrite(GUEST_GS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_GS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_GS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(
            GUEST_GS_BASE,
            self.host_state.SpecialRegisters.MsrGsBase as _,
        ); // fuck
        __vmx_vmwrite(
            HOST_GS_BASE,
            self.host_state.SpecialRegisters.MsrGsBase as _,
        );
        __vmx_vmwrite(
            HOST_GS_SELECTOR,
            (self.host_state.Context_frame.SegGs & !3) as _,
        );

        // ss
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.Context_frame.SegSs,
        );
        __vmx_vmwrite(GUEST_SS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_SS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_SS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_SS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(
            HOST_SS_SELECTOR,
            (self.host_state.Context_frame.SegSs & !3) as _,
        );

        // tr
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.SpecialRegisters.Tr,
        );
        __vmx_vmwrite(GUEST_TR_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_TR_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_TR_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_TR_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_TR_BASE, gdt_entry.base as _);
        __vmx_vmwrite(
            HOST_TR_SELECTOR,
            (self.host_state.SpecialRegisters.Tr & !3) as _,
        );

        // ldtr,host no ldtr,only gdt
        let gdt_entry = self.convert_gdt_entry(
            self.host_state.SpecialRegisters.Gdtr.Base,
            self.host_state.SpecialRegisters.Ldtr,
        );
        __vmx_vmwrite(GUEST_LDTR_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_LDTR_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_LDTR_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _
        });
        __vmx_vmwrite(GUEST_LDTR_BASE, gdt_entry.base as _);

        // GDT
        __vmx_vmwrite(GUEST_GDTR_BASE, self.host_state.SpecialRegisters.Gdtr.Base);
        __vmx_vmwrite(
            GUEST_GDTR_LIMIT,
            self.host_state.SpecialRegisters.Gdtr.Limit as _,
        );
        __vmx_vmwrite(HOST_GDTR_BASE, self.host_state.SpecialRegisters.Gdtr.Base);

        // IDT
        __vmx_vmwrite(GUEST_IDTR_BASE, self.host_state.SpecialRegisters.Idtr.Base);
        __vmx_vmwrite(
            GUEST_IDTR_LIMIT,
            self.host_state.SpecialRegisters.Idtr.Limit as _,
        );
        __vmx_vmwrite(HOST_IDTR_BASE, self.host_state.SpecialRegisters.Idtr.Base);

        // CR0
        __vmx_vmwrite(CR0_GUEST_HOST_MASK, 0xffffffff);
        __vmx_vmwrite(CR0_READ_SHADOW, self.host_state.SpecialRegisters.Cr0);
        __vmx_vmwrite(HOST_CR0, self.host_state.SpecialRegisters.Cr0);
        __vmx_vmwrite(GUEST_CR0, self.host_state.SpecialRegisters.Cr0);

        // CR3
        __vmx_vmwrite(HOST_CR3, self.host_state.SpecialRegisters.Cr3);
        __vmx_vmwrite(GUEST_CR3, self.host_state.SpecialRegisters.Cr3);

        // CR4
        __vmx_vmwrite(HOST_CR4, self.host_state.SpecialRegisters.Cr4);
        __vmx_vmwrite(GUEST_CR4, self.host_state.SpecialRegisters.Cr4);
        __vmx_vmwrite(CR4_GUEST_HOST_MASK, 0x2000);
        __vmx_vmwrite(
            CR4_READ_SHADOW,
            self.host_state.SpecialRegisters.Cr4 & !0x2000,
        );

        // Debug MSR and DR7
        __vmx_vmwrite(
            GUEST_IA32_DEBUGCTL,
            self.host_state.SpecialRegisters.DebugControl,
        );
        __vmx_vmwrite(GUEST_DR7, self.host_state.SpecialRegisters.KernelDr7);

        // guest address after execute vm_launch
        __vmx_vmwrite(GUEST_RSP, self.host_state.Context_frame.Rsp);
        __vmx_vmwrite(GUEST_RIP, self.host_state.Context_frame.Rip);
        __vmx_vmwrite(GUEST_RFLAGS, self.host_state.Context_frame.EFlags as _);

        // vmm entrypoint and stack address
        __vmx_vmwrite(
            HOST_RSP,
            (self.vm_resources.vmm_stack as u64 + KERNEL_STACK_SIZE as u64 - 8 * 2) as _,
        );
        __vmx_vmwrite(HOST_RIP, vmm_entry_point as _);
    }

    fn subvert_cpu(&mut self) {
        let mut phys: PHYSICAL_ADDRESS = PHYSICAL_ADDRESS::default();
        phys.QuadPart = -1;

        // need free it youself on drop fuction
        let vmxon = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _, phys) };
        let vmcs = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _, phys) };
        let vmm_stack = unsafe { MmAllocateContiguousMemory(KERNEL_STACK_SIZE as _, phys) };
        let msr_bitmap = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _, phys) };

        // allocate fault
        if vmxon.is_null() || vmcs.is_null() || vmm_stack.is_null() {
            return;
        }

        self.vm_resources.vmxon = vmxon as _;
        self.vm_resources.vmcs = vmcs as _;
        self.vm_resources.vmm_stack = vmm_stack;
        self.vm_resources.msr_bitmap = msr_bitmap;

        // set physical page RW
        unsafe {
            if protect_non_paged_memory(vmxon, size_of::<VmxVmcs>() as _, PAGE_READWRITE).is_err() {
                return;
            }

            if protect_non_paged_memory(vmcs, size_of::<VmxVmcs>() as _, PAGE_READWRITE).is_err() {
                return;
            }

            if protect_non_paged_memory(vmm_stack, KERNEL_STACK_SIZE as _, PAGE_READWRITE).is_err()
            {
                return;
            }

            if protect_non_paged_memory(msr_bitmap, PAGE_SIZE as _, PAGE_READWRITE).is_err() {
                return;
            }
        }

        // zero memory
        unsafe {
            core::ptr::write_bytes(vmxon, 0, size_of::<VmxVmcs>());
            core::ptr::write_bytes(vmcs, 0, size_of::<VmxVmcs>());
            core::ptr::write_bytes(vmm_stack, 0, KERNEL_STACK_SIZE as _);
            core::ptr::write_bytes(msr_bitmap, 0, PAGE_SIZE as _);
        }

        // enter vmx root
        match self.enter_vmx_root_mode() {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e);
                return;
            }
        }

        info!("already enter vmx root mode");

        self.set_vmcs_data();

        self.vcpu_vmx_state = VcpuVmxState::VmxStateTransition;

        // vm-entry by execute vmlaunch instruction
        // from vmm to guest
        __vmx_vmlaunch();

        error!("Vmlaunch error:{}", __vmx_read_error());

        // this signifies an error occurrence if reaches next code during execution
        if self.vmxon {
            match __vmx_off() {
                VmxInstructionResult::VmxSuccess => {
                    self.vmxon = false;
                    self.vcpu_vmx_state = VcpuVmxState::VmxStateOff;
                    info!("already exit vmx root mode");
                }
                _ => {
                    error!("Vmxoff execute error:{}", __vmx_read_error());
                }
            }
        }
    }

    fn start_vt(&mut self) {
        unsafe {
            let host_state: &mut KPROCESSOR_STATE = &mut self.host_state;
            KeSaveStateForHibernate(host_state as _);

            // important!!!!
            // continue on next code after execute vmx_on instruction
            RtlCaptureContext(&mut host_state.Context_frame as _);
        }

        match self.vcpu_vmx_state {
            VcpuVmxState::VmxStateOff => {
                // begin start vt
                self.subvert_cpu();
            }
            VcpuVmxState::VmxStateTransition => {
                // vmlauch execute successed
                self.vcpu_vmx_state = VcpuVmxState::VmxStateOn;
                unsafe { RtlRestoreContext(&mut self.host_state.Context_frame as _, null_mut()) };
            }
            VcpuVmxState::VmxStateOn => {
                // all success
                info!("CPU:{} start vt success", self.cpu_index);
            }
        }
    }

    pub fn set_vmx_state(&mut self, state: VcpuVmxState) {
        self.vcpu_vmx_state = state;
        if self.vcpu_vmx_state == VcpuVmxState::VmxStateOff {
            self.vmxon = false;
        }
    }

    pub fn set_cpu_index(&mut self, index: usize) {
        self.cpu_index = index;
    }
}

impl Drop for Vcpu {
    fn drop(&mut self) {}
}

impl Default for Vmm {
    fn default() -> Self {
        Self::new()
    }
}

impl Vmm {
    pub fn new() -> Self {
        let cpu_count = unsafe { KeQueryActiveProcessorCount(core::ptr::null_mut()) } as u32;

        info!("cpu_count:{}", cpu_count);

        let mut vcpus: Vec<Box<Vcpu>> = Vec::with_capacity(cpu_count as _);

        for _ in 0..cpu_count {
            let vcpu = Vcpu {
                host_state: Box::new(KPROCESSOR_STATE::default()),
                vcpu_vmx_state: VcpuVmxState::VmxStateOff,
                vm_resources: VmcsResources {
                    vmxon: core::ptr::null_mut(),
                    vmcs: core::ptr::null_mut(),
                    vmm_stack: core::ptr::null_mut(),
                    msr_bitmap: core::ptr::null_mut(),
                },
                vmxon: false,
                cpu_index: 0,
            };
            let v = Box::new(vcpu);
            vcpus.push(v);
        }

        Self {
            cpu_count,
            vmx_features: VMXFeatures::default(),
            ept_state: Option::None,
            vcpu: vcpus,
        }
    }

    fn check_and_set_features(&mut self) {
        let vmx_basic = read_msr(MSR_IA32_VMX_BASIC);
        self.vmx_features.true_msrs = (vmx_basic & VMX_BASIC_TRUE_CTLS) != 0;

        let mut vmx_proc = read_msr(MSR_IA32_VMX_PROCBASED_CTLS) >> 32;
        if self.vmx_features.true_msrs {
            vmx_proc = read_msr(MSR_IA32_VMX_TRUE_PROCBASED_CTLS) >> 32;
        }

        self.vmx_features.secondary_controls =
            (vmx_proc & VMX_PROC_CTLS_USE_SECONDARY_CTLS as u64) != 0;

        if self.vmx_features.secondary_controls {
            let vmx_proc2 = read_msr(MSR_IA32_VMX_PROCBASED_CTLS2) >> 32;

            self.vmx_features.ept = (vmx_proc2 & VMX_PROC_CTLS2_EPT as u64) != 0;
            self.vmx_features.vpid = (vmx_proc2 & VMX_PROC_CTLS2_VPID as u64) != 0;
            self.vmx_features.vmfunc = (vmx_proc2 & VMX_PROC_CTLS2_VMFUNC as u64) != 0;

            if self.vmx_features.ept {
                let ept_vpid_cap = read_msr(MSR_IA32_VMX_EPT_VPID_CAP);
                self.vmx_features.exec_only_ept = (ept_vpid_cap
                    & ia32_vmx_ept_vpid_cap_msr::MSR_IA32_VMX_EPT_VPID_CAP_RWX_X_ONLY)
                    != 0;
                self.vmx_features.inv_single_address = (ept_vpid_cap
                    & ia32_vmx_ept_vpid_cap_msr::MSR_IA32_VMX_EPT_VPID_CAP_INVVPID_INDIV_ADDR)
                    != 0;
            }
        }

        if in_vmware() {
            self.vmx_features.in_vmware = true;
        }
    }

    pub fn start(&mut self) -> Result<(), StartVTError> {
        self.check_and_set_features();
        if self.vmx_features.ept {
            self.ept_state = Some(EptState::new());
        }

        for i in 0..self.cpu_count {
            unsafe { KeSetSystemAffinityThread(1 << i) };
            let vcpu = &mut self.vcpu[i as usize];
            vcpu.set_cpu_index(i as _);
            vcpu.start_vt();
            unsafe { KeRevertToUserAffinityThread() };
        }

        for item in &self.vcpu {
            if item.vcpu_vmx_state != VcpuVmxState::VmxStateOn {
                return Err(StartVTError {});
            }
        }

        Ok(())
    }

    pub fn get_current_vcpu(&mut self) -> &mut Vcpu {
        &mut self.vcpu[get_current_processor_idx() as usize]
    }
}

impl Drop for Vmm {
    fn drop(&mut self) {
        for cvcpu in &mut self.vcpu {
            match cvcpu.vcpu_vmx_state {
                VcpuVmxState::VmxStateOn => {
                    unsafe { KeSetSystemAffinityThread(1 << cvcpu.cpu_index) };

                    match __vmx_vmcall(EXIT_VT, 0, 0, 0) {
                        VmxInstructionResult::VmxSuccess => {
                            info!("CPU:{} Close VT Success", cvcpu.cpu_index);
                        }
                        _ => {
                            error!("Vmxcall execute error");
                        }
                    }

                    unsafe { KeRevertToUserAffinityThread() };
                }
                _ => {
                    if cvcpu.vmxon {
                        unsafe { KeSetSystemAffinityThread(1 << cvcpu.cpu_index) };
                        info!("vmxoff exec");
                        match __vmx_off() {
                            VmxInstructionResult::VmxSuccess => {}
                            _ => {
                                error!("Vmxoff execute error");
                            }
                        }
                        unsafe { KeRevertToUserAffinityThread() };
                    }
                }
            }

            cvcpu.free_physical_memory();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VcpuVmxState {
    VmxStateOff,
    VmxStateTransition,
    VmxStateOn,
}

#[repr(C)]
#[derive(Debug)]

struct VmxVmcs {
    revision_id: u32,     // version
    abort_indicator: u32, // vmx abort reason. vmx abort:vmexit fault
    data: [u8; PAGE_SIZE - 2 * core::mem::size_of::<u32>()],
}
const PAGE_SIZE: usize = 4096;

#[derive(Default)]
pub struct VMXFeatures {
    pub secondary_controls: bool, // Secondary controls are enabled
    pub true_msrs: bool,          // True VMX MSR values are supported
    pub ept: bool,                // EPT supported by CPU
    pub vpid: bool,               // VPID supported by CPU
    pub exec_only_ept: bool,      // EPT translation with execute-only access is supported
    pub inv_single_address: bool, // IVVPID for single address
    pub vmfunc: bool,             // VMFUNC is supported
    pub in_vmware: bool,
    // meltdown: bool,                 // intel meltdown
    // spectre: bool,                  // intel and amd spectre
}
