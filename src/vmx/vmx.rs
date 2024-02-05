use core::{arch::asm, ffi::c_void, mem::size_of};

use alloc::vec::Vec;
use wdk::println;
use wdk_sys::{ntddk::{IoAllocateMdl, IoFreeMdl, KeGetCurrentProcessorNumberEx, KeQueryActiveProcessorCount, KeRevertToUserAffinityThread, KeSetSystemAffinityThread, MmAllocateContiguousMemory, MmBuildMdlForNonPagedPool, MmFreeContiguousMemory, MmGetPhysicalAddress, MmProtectMdlSystemAddress, RtlCaptureContext, RtlInitializeBitMap, RtlSetBit}, CONTEXT, KERNEL_STACK_SIZE, MDL_MAPPED_TO_SYSTEM_VA, NT_SUCCESS, PAGE_READWRITE, PHYSICAL_ADDRESS, RTL_BITMAP, USHORT, _LARGE_INTEGER};

use crate::{cpu::cpu::{ins::{read_cr3, read_msr, segment_limit, write_cr0, write_cr4}, stru::msr_index::{MSR_IA32_DEBUGCTL, MSR_IA32_FEATURE_CONTROL, MSR_IA32_VMX_BASIC, MSR_IA32_VMX_CR0_FIXED0, MSR_IA32_VMX_CR0_FIXED1, MSR_IA32_VMX_CR4_FIXED0, MSR_IA32_VMX_CR4_FIXED1, MSR_IA32_VMX_PROCBASED_CTLS2, MSR_IA32_VMX_TRUE_ENTRY_CTLS, MSR_IA32_VMX_TRUE_EXIT_CTLS, MSR_IA32_VMX_TRUE_PINBASED_CTLS, MSR_IA32_VMX_TRUE_PROCBASED_CTLS, MSR_IA32_VMX_VMFUNC, MSR_LSTAR}}, inner::{data_struct::KPROCESSOR_STATE, KeSaveStateForHibernate}, utils::utils::{__debugbreak, create_end_mask, get_bits_value, set_bits_value}, vmx::{data::vmcs_encoding::VM_INSTRUCTION_ERROR, ins::{__vmx_on, __vmx_vmclear, __vmx_vmlaunch, __vmx_vmptrld, __vmx_vmread}}};

use super::{data::{vmcs_encoding::{CPU_BASED_VM_EXEC_CONTROL, CR0_READ_SHADOW, CR4_GUEST_HOST_MASK, CR4_READ_SHADOW, GUEST_CR0, GUEST_CR3, GUEST_CR4, GUEST_CS_AR_BYTES, GUEST_CS_BASE, GUEST_CS_LIMIT, GUEST_CS_SELECTOR, GUEST_DR7, GUEST_DS_AR_BYTES, GUEST_DS_BASE, GUEST_DS_LIMIT, GUEST_DS_SELECTOR, GUEST_ES_AR_BYTES, GUEST_ES_BASE, GUEST_ES_LIMIT, GUEST_ES_SELECTOR, GUEST_FS_AR_BYTES, GUEST_FS_BASE, GUEST_FS_LIMIT, GUEST_FS_SELECTOR, GUEST_GDTR_BASE, GUEST_GDTR_LIMIT, GUEST_GS_AR_BYTES, GUEST_GS_BASE, GUEST_GS_LIMIT, GUEST_GS_SELECTOR, GUEST_IA32_DEBUGCTL, GUEST_IDTR_BASE, GUEST_IDTR_LIMIT, GUEST_LDTR_AR_BYTES, GUEST_LDTR_BASE, GUEST_LDTR_LIMIT, GUEST_LDTR_SELECTOR, GUEST_RFLAGS, GUEST_RIP, GUEST_RSP, GUEST_SS_AR_BYTES, GUEST_SS_BASE, GUEST_SS_LIMIT, GUEST_SS_SELECTOR, GUEST_TR_AR_BYTES, GUEST_TR_BASE, GUEST_TR_LIMIT, GUEST_TR_SELECTOR, HOST_CR0, HOST_CR3, HOST_CR4, HOST_CS_SELECTOR, HOST_DS_SELECTOR, HOST_ES_SELECTOR, HOST_FS_SELECTOR, HOST_GDTR_BASE, HOST_GS_SELECTOR, HOST_IDTR_BASE, HOST_RIP, HOST_RSP, HOST_SS_SELECTOR, HOST_TR_SELECTOR, MSR_BITMAP, PIN_BASED_VM_EXEC_CONTROL, SECONDARY_VM_EXEC_CONTROL, VMCS_LINK_POINTER, VM_ENTRY_CONTROLS, VM_EXIT_CONTROLS}, vmx_cpu_based_controls, vmx_secondary_cpu_based_controls, vmx_vm_enter_controls, vmx_vm_exit_controls}, ins::{VmxInstructionResult, __vmx_off, __vmx_vmwrite}};


extern "C"{
    pub fn vmm_entry_point();
}


struct VmcsResources {
    vmxon: *mut VmxVmcs,
    vmcs: *mut VmxVmcs,
    vmm_stack: *mut c_void,
    msr_bitmap: *mut c_void,
}

pub struct Vcpu {
    host_state: KPROCESSOR_STATE,
    vcpu_vmx_state: VcpuVmxState,
    vm_resources: VmcsResources,
    vmxon: bool,
}

pub struct Vmm {
    cpu_count: u32,
    vmx_features: VMXFeatures,
    vcpu: Vec<Vcpu>,
}

impl Vcpu {
    // free vmm relate physical memory self
    pub fn free_physical_memory(&mut self) {
        println!("free_physical_memory");
        
        let vmcs_resources = &mut self.vm_resources;
        let vmxon = &mut vmcs_resources.vmxon;
        let vmcs = &mut vmcs_resources.vmcs;
        let vmm_stack = &mut vmcs_resources.vmm_stack;

        if !vmxon.is_null(){
            unsafe { MmFreeContiguousMemory(core::mem::replace(vmxon, core::ptr::null_mut()) as _) };
        }
        if !vmcs.is_null(){
            unsafe { MmFreeContiguousMemory(core::mem::replace(vmcs, core::ptr::null_mut()) as _) };
        }
        if !vmm_stack.is_null(){
            unsafe { MmFreeContiguousMemory(core::mem::replace(vmm_stack, core::ptr::null_mut()) as _) };
        }
    }

    fn enter_vmx_root_mode(&mut self) -> Result<(),& 'static str>{
        let vmx_basic = read_msr(MSR_IA32_VMX_BASIC);
        let cr0_fixed0 = read_msr(MSR_IA32_VMX_CR0_FIXED0);
        let cr0_fixed1 = read_msr(MSR_IA32_VMX_CR0_FIXED1);
        let cr4_fixed0 = read_msr(MSR_IA32_VMX_CR4_FIXED0);
        let cr4_fixed1 = read_msr(MSR_IA32_VMX_CR4_FIXED1);

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
        let phys = unsafe{ 
            &mut MmGetPhysicalAddress(self.vm_resources.vmxon as *mut c_void) as *mut _LARGE_INTEGER
        };

        match __vmx_on(phys as _) {
            VmxInstructionResult::VmxSuccess => {},
            _ => {
                return Err("vmxon execute fault");
            }
        }

        self.vmxon = true;

        // vmclear vmcs
        let phys = unsafe{ 
            &mut MmGetPhysicalAddress(self.vm_resources.vmcs as *mut c_void) as *mut _LARGE_INTEGER
        };

        match __vmx_vmclear(phys as _) {
            VmxInstructionResult::VmxSuccess => {},
            _ => {
                return Err("vmclear execute fault");
            }
        }

        // vmptrld
        match __vmx_vmptrld(phys as _) {
            VmxInstructionResult::VmxSuccess => {},
            _ => {
                return Err("vmclear execute fault");
            }
        }

        return Ok(())
    }   

    fn vmxp_adjust_msr(&self,control_value: u64, desired_value: u64) -> u64 {
        let mut result;
        result = desired_value & (control_value >> 32);
        result |= (control_value as u32) as u64;
        return result;
    }

    fn convert_gdt_entry(&mut self,base: u64,selector: USHORT) -> GdtEntry64 {
        // limit 
        let limit = segment_limit(selector as _);

        // transform raw point to struct refrence
        let gdt_entry: *mut KGDTENTRY64 = (base + (selector as u64 & !3)) as *mut _;
        let gdt_entry = unsafe { &mut *gdt_entry };

        // base
        let mut temp_base;

        temp_base = unsafe {
            (((gdt_entry.dummy.u.bytes.BaseHigh) as u64) << 24) as u64 | 
            (((gdt_entry.dummy.u.bytes.BaseMiddle) as u64) << 16) as u64 | 
            ((gdt_entry.dummy.base_low)) as u64 & u64::MAX
        };

        if (get_bits_value(unsafe{gdt_entry.dummy.u.bits} as _, 8, 5) & 0x10) == 0 {
            temp_base |= unsafe{(gdt_entry.dummy.base_upper as u64) << 32}
        }else{
            temp_base |= 0;
        }

        // access right
        let mut access_rights:GDTENTRY64_ACCESS_RIGHTS = GDTENTRY64_ACCESS_RIGHTS{access_rights: 0u32};

        unsafe { access_rights.bytes.flags1 = gdt_entry.dummy.u.bytes.Flags1 };
        unsafe { access_rights.bytes.flags2 = gdt_entry.dummy.u.bytes.Flags2 };

        // USHORT Reserved : 4;
        unsafe {
            access_rights.bits = set_bits_value( access_rights.bits as u64, 8, 4, 0) as u16
        };

        // Unusable = !Present
        if (get_bits_value(unsafe { gdt_entry.dummy.u.bits as _}, 15, 1)) == 0 {
            unsafe{
                access_rights.access_rights = set_bits_value(access_rights.bits as _, 16, 1, 1) as _
            };
        }else{
            unsafe{
                access_rights.access_rights = set_bits_value(access_rights.bits as _, 16, 1, 0) as _
            };
        }

        // println!("gdtentry:{:X},{:X},{:X},{:X}",selector,limit,unsafe {
        //     access_rights.access_rights    
        // },temp_base);

        return GdtEntry64{
            selector,
            limit: limit as _,
            access_rights,
            base: temp_base,
        }
    }   

    // todo
    fn set_vmcs_data(&mut self) {
        let vmx_pin = read_msr(MSR_IA32_VMX_TRUE_PINBASED_CTLS);
        let vmx_cpu = read_msr(MSR_IA32_VMX_TRUE_PROCBASED_CTLS);
        //cpu secondary
        let vmx_sec = read_msr(MSR_IA32_VMX_PROCBASED_CTLS2);
        //VM Exit
        let vmx_exit = read_msr(MSR_IA32_VMX_TRUE_EXIT_CTLS);
        //VM Entry
        let vmx_entry = read_msr(MSR_IA32_VMX_TRUE_ENTRY_CTLS);

        let mut vm_pin_ctl_requested:u64 = 0;
        let mut vm_cpu_ctl_requested:u64 = 0;
        let mut vm_cpu_ctl2_requested:u64 = 0;
        let mut vm_enter_ctl_requested:u64 = 0;
        let mut vm_exit_ctl_requested:u64 = 0;

        vm_cpu_ctl_requested |= vmx_cpu_based_controls::ACTIVATE_SECONDARY_CONTROL;
        vm_cpu_ctl_requested |= vmx_cpu_based_controls::USE_MSR_BITMAPS; // msr

        vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::ENABLE_RDTSCP;
        vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::ENABLE_INVPCID;
        vm_cpu_ctl2_requested |= vmx_secondary_cpu_based_controls::ENABLE_XSAVESX_STORS;

        vm_enter_ctl_requested |= vmx_vm_enter_controls::LOAD_DEBUG_CONTROLS; // dr
        vm_enter_ctl_requested |= vmx_vm_enter_controls::IA32E_MODE_GUEST;
        
        vm_exit_ctl_requested |= vmx_vm_exit_controls::HOST_ADDRESS_SPACE_SIZE;


        // todo msr bitmap
        let bit_map_read_low: *mut u32 = self.vm_resources.msr_bitmap as _;
        let bit_map_read_high: *mut u32 = (bit_map_read_low as u64 + 1024) as _;
        let bit_map_write_low: *mut u32 = (bit_map_read_high as u64 + 1024) as _;
        let bit_map_write_high: *mut u32 = (bit_map_write_low as u64 + 1024) as _;

        let mut bit_map_read_low_header:RTL_BITMAP = RTL_BITMAP{ SizeOfBitMap: 0, Buffer: core::ptr::null_mut() };
        let mut bit_map_read_high_header:RTL_BITMAP = RTL_BITMAP{ SizeOfBitMap: 0, Buffer: core::ptr::null_mut() };
        let mut bit_map_write_low_header:RTL_BITMAP = RTL_BITMAP{ SizeOfBitMap: 0, Buffer: core::ptr::null_mut() };
        let mut bit_map_write_high_header:RTL_BITMAP = RTL_BITMAP{ SizeOfBitMap: 0, Buffer: core::ptr::null_mut() };
        
        unsafe {
            RtlInitializeBitMap(&mut bit_map_read_low_header as _, bit_map_read_low, 1024 * 8);
            RtlInitializeBitMap(&mut bit_map_read_high_header as _, bit_map_read_high, 1024 * 8);
            RtlInitializeBitMap(&mut bit_map_write_low_header as _, bit_map_write_low, 1024 * 8);
            RtlInitializeBitMap(&mut bit_map_write_high_header as _, bit_map_write_high, 1024 * 8);   
        }

        unsafe{
            RtlSetBit(&mut bit_map_read_low_header as _, MSR_IA32_FEATURE_CONTROL);
            RtlSetBit(&mut bit_map_read_low_header as _, MSR_IA32_DEBUGCTL);
            RtlSetBit(&mut bit_map_read_high_header as _, MSR_LSTAR - 0xC0000000);
            
            RtlSetBit(&mut bit_map_write_low_header as _, MSR_IA32_FEATURE_CONTROL);
            RtlSetBit(&mut bit_map_write_low_header as _, MSR_IA32_DEBUGCTL);
            RtlSetBit(&mut bit_map_write_high_header as _, MSR_LSTAR - 0xC0000000);
        }

        for i in MSR_IA32_VMX_BASIC..=MSR_IA32_VMX_VMFUNC {
            unsafe{
                RtlSetBit(&mut bit_map_read_low_header, i);
                RtlSetBit(&mut bit_map_write_low_header, i);
            }
        }

        __vmx_vmwrite(MSR_BITMAP, unsafe {
            MmGetPhysicalAddress(self.vm_resources.msr_bitmap).QuadPart as _   
        });
        

        __vmx_vmwrite(VMCS_LINK_POINTER as _, u64::MAX);

        //Secondary,todo:
        __vmx_vmwrite(SECONDARY_VM_EXEC_CONTROL as _,
            self.vmxp_adjust_msr(vmx_sec, vm_cpu_ctl2_requested)
        );

        //PIN
        __vmx_vmwrite(
            PIN_BASED_VM_EXEC_CONTROL as _,
            self.vmxp_adjust_msr(vmx_pin, vm_pin_ctl_requested)
        );

        //CPU,todo
        __vmx_vmwrite(
            CPU_BASED_VM_EXEC_CONTROL as _,
            self.vmxp_adjust_msr(vmx_cpu, vm_cpu_ctl_requested)
        );

        //VM Exit
        __vmx_vmwrite(
            VM_EXIT_CONTROLS as _,
            self.vmxp_adjust_msr(vmx_exit, vm_exit_ctl_requested)
        );

        //VM Entry,todo
        __vmx_vmwrite(
            VM_ENTRY_CONTROLS as _,
            self.vmxp_adjust_msr(vmx_entry, vm_enter_ctl_requested)
        );
        
        // cs
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.context_frame.SegCs);
        __vmx_vmwrite(GUEST_CS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_CS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_CS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_CS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_CS_SELECTOR, (self.host_state.context_frame.SegCs & !3) as _);

        // ss
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.context_frame.SegSs);
        __vmx_vmwrite(GUEST_SS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_SS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_SS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_SS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_SS_SELECTOR, (self.host_state.context_frame.SegSs & !3) as _);

        // ds
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.context_frame.SegDs);
        __vmx_vmwrite(GUEST_DS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_DS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_DS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_DS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_DS_SELECTOR, (self.host_state.context_frame.SegDs & !3) as _);

        // es
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.context_frame.SegEs);
        __vmx_vmwrite(GUEST_ES_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_ES_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_ES_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_ES_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_ES_SELECTOR, (self.host_state.context_frame.SegEs & !3) as _);

        // fs
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.context_frame.SegFs);
        __vmx_vmwrite(GUEST_FS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_FS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_FS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_FS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_FS_SELECTOR, (self.host_state.context_frame.SegFs & !3) as _);

        // gs
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.context_frame.SegGs);
        __vmx_vmwrite(GUEST_GS_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_GS_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_GS_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_GS_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_GS_SELECTOR, (self.host_state.context_frame.SegGs & !3) as _);

        // tr
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.SpecialRegisters.Tr);
        __vmx_vmwrite(GUEST_TR_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_TR_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_TR_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_TR_BASE, gdt_entry.base as _);
        __vmx_vmwrite(HOST_TR_SELECTOR, (self.host_state.SpecialRegisters.Tr & !3) as _);

        // ldtr todo
        let gdt_entry = self.convert_gdt_entry(self.host_state.SpecialRegisters.Gdtr.base, self.host_state.SpecialRegisters.Ldtr);
        __vmx_vmwrite(GUEST_LDTR_SELECTOR, gdt_entry.selector as _);
        __vmx_vmwrite(GUEST_LDTR_LIMIT, gdt_entry.limit as _);
        __vmx_vmwrite(GUEST_LDTR_AR_BYTES, unsafe {
            gdt_entry.access_rights.access_rights as _  
        });
        __vmx_vmwrite(GUEST_LDTR_BASE, gdt_entry.base as _);

        // GDT
        __vmx_vmwrite(GUEST_GDTR_BASE, self.host_state.SpecialRegisters.Gdtr.base);
        __vmx_vmwrite(GUEST_GDTR_LIMIT, self.host_state.SpecialRegisters.Gdtr.Limit as _);
        __vmx_vmwrite(HOST_GDTR_BASE, self.host_state.SpecialRegisters.Gdtr.base);

        // IDT
        __vmx_vmwrite(GUEST_IDTR_BASE, self.host_state.SpecialRegisters.Idtr.base);
        __vmx_vmwrite(GUEST_IDTR_LIMIT, self.host_state.SpecialRegisters.Idtr.Limit as _);
        __vmx_vmwrite(HOST_IDTR_BASE, self.host_state.SpecialRegisters.Idtr.base);

        // CR0
        __vmx_vmwrite(CR0_READ_SHADOW, self.host_state.SpecialRegisters.Cr0);
        __vmx_vmwrite(HOST_CR0, self.host_state.SpecialRegisters.Cr0);
        __vmx_vmwrite(GUEST_CR0, self.host_state.SpecialRegisters.Cr0);

        // cr3
        __vmx_vmwrite(HOST_CR3,  read_cr3());
        __vmx_vmwrite(GUEST_CR3, read_cr3());

        // CR4
        __vmx_vmwrite(HOST_CR4, self.host_state.SpecialRegisters.Cr4);
        __vmx_vmwrite(GUEST_CR4, self.host_state.SpecialRegisters.Cr4);
        __vmx_vmwrite(CR4_GUEST_HOST_MASK, 0x2000);
        __vmx_vmwrite(CR4_READ_SHADOW, self.host_state.SpecialRegisters.Cr4 & !0x2000);

        // Debug MSR and DR7
        __vmx_vmwrite(GUEST_IA32_DEBUGCTL, self.host_state.SpecialRegisters.DebugControl);
        __vmx_vmwrite(GUEST_DR7, self.host_state.SpecialRegisters.KernelDr7);

        // guest address after execute vm_launch
        __vmx_vmwrite(GUEST_RSP, self.host_state.context_frame.Rsp);
        __vmx_vmwrite(GUEST_RIP, self.host_state.context_frame.Rip);
        __vmx_vmwrite(GUEST_RFLAGS, self.host_state.context_frame.EFlags as _);

        // vmm entrypoint and stack address
        __vmx_vmwrite(HOST_RSP, (self.vm_resources.vmm_stack as u64 + KERNEL_STACK_SIZE as u64 - 8 * 2) as _);
        __vmx_vmwrite(HOST_RIP, vmm_entry_point as _);

    }

    fn subvert_cpu(&mut self) {
        // force cast
        let phys:PHYSICAL_ADDRESS = unsafe { core::mem::transmute(&u64::MAX) };

        // need free it youself on drop fuction
        let vmxon = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _,phys) };
        let vmcs = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _,phys) };
        let vmm_stack = unsafe { MmAllocateContiguousMemory(KERNEL_STACK_SIZE as _,phys) };
        let msr_bitmap = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _,phys) };

        // allocate fault
        if vmxon.is_null() || vmcs.is_null() || vmm_stack.is_null() {
            return;
        }

        self.vm_resources.vmxon = vmxon as _;
        self.vm_resources.vmcs = vmcs as _;
        self.vm_resources.vmm_stack = vmm_stack;
        self.vm_resources.msr_bitmap = msr_bitmap;

        // set physical page RW
        match protect_non_paged_memory(vmxon,size_of::<VmxVmcs>() as _,PAGE_READWRITE) {
            Ok(_) => {},
            Err(_) => {
                return;
            },
        }
        match protect_non_paged_memory(vmcs,size_of::<VmxVmcs>() as _,PAGE_READWRITE) {
            Ok(_) => {},
            Err(_) => {
                return;
            },
        }
        match protect_non_paged_memory(vmm_stack,KERNEL_STACK_SIZE as _,PAGE_READWRITE) {
            Ok(_) => {},
            Err(_) => {
                return;
            },
        }
        match protect_non_paged_memory(msr_bitmap,PAGE_SIZE as _,PAGE_READWRITE) {
            Ok(_) => {},
            Err(_) => {
                return;
            },
        }

        // zero memory
        unsafe{
            vmxon.write_bytes(0, size_of::<VmxVmcs>());
            vmcs.write_bytes(0, size_of::<VmxVmcs>());
            vmm_stack.write_bytes(0, KERNEL_STACK_SIZE as _);
            msr_bitmap.write_bytes(0, PAGE_SIZE as _);
        }

        // enter vmx root
        match self.enter_vmx_root_mode() {
            Ok(_) =>{}
            Err(e) => {
                println!("{}",e);
                return;
            }
        }

        println!("already enter vmx root mode");

        self.set_vmcs_data();

        self.vcpu_vmx_state = VcpuVmxState::VmxStateTransition;

        match __vmx_vmlaunch(){
            VmxInstructionResult::VmxSuccess => todo!(),
            _ => {
                // read launch error code
                let mut error_code:u64 = 0;
                match __vmx_vmread(VM_INSTRUCTION_ERROR,&mut error_code) {
                    VmxInstructionResult::VmxSuccess => {
                        println!("Read ins error code success:{}",error_code);
                    },
                    _ => {
                        println!("error to read vmlaunch error code");
                    }
                }
            },
        }

        println!("Vmlaunch error");

        // this signifies an error occurrence if reaches next code during execution
        if self.vmxon{
            __vmx_off();
            self.vmxon = false;
            self.vcpu_vmx_state = VcpuVmxState::VmxStateOff;
        }

    }

    fn start_vt(&mut self) {
        let host_state = &mut self.host_state;
        let host_state_ptr: *mut KPROCESSOR_STATE = host_state;
        let context_frame_ptr: *mut CONTEXT = &mut host_state.context_frame;

        unsafe{ KeSaveStateForHibernate(host_state_ptr); }

        // important!!!!
        // continue on next code after execute vmx_on instruction
        unsafe { RtlCaptureContext(context_frame_ptr); }

        match self.vcpu_vmx_state {
            VcpuVmxState::VmxStateOff => {
                // begin start vt
                self.subvert_cpu();
            }
            VcpuVmxState::VmxStateTransition => {
                // vmlauch execute successed 
                // todo
                println!("VmxStateTransition");
                self.vcpu_vmx_state = VcpuVmxState::VmxStateOn;
            },
            VcpuVmxState::VmxStateOn => {
                // all success
                let current_cpu_index = unsafe { KeGetCurrentProcessorNumberEx(core::ptr::null_mut()) };
                println!("CPU:{} start vt success",current_cpu_index);
            },
        }
    }
}

impl Vmm {
    pub fn new() -> Self {
        println!("VmxData new222");

        let cpu_count = unsafe { KeQueryActiveProcessorCount(core::ptr::null_mut()) } as u32;

        println!("cpu_count:{}", cpu_count);

        let mut vcpus: Vec<Vcpu> = Vec::with_capacity(cpu_count as _);

        for _ in 0..cpu_count {
            let vcpu = Vcpu {
                host_state: KPROCESSOR_STATE::default(),
                vcpu_vmx_state: VcpuVmxState::VmxStateOff,
                vm_resources: VmcsResources { 
                    vmxon: core::ptr::null_mut(),
                    vmcs: core::ptr::null_mut(),
                    vmm_stack: core::ptr::null_mut(),
                    msr_bitmap: core::ptr::null_mut(),
                },
                vmxon: false,
            };
            vcpus.push(vcpu);
        }

        Self {
            cpu_count: cpu_count,
            vmx_features: VMXFeatures::default(),
            vcpu: vcpus,
        }
    }

    // todo
    fn check_and_set_features(&self) {

    }

    pub fn init(&mut self) {
        println!("Start to check vmx features");
        self.check_and_set_features();

        for i in 0..self.cpu_count {
            unsafe { KeSetSystemAffinityThread(1 << i) };
            let vcpu = &mut self.vcpu[i as usize];
            vcpu.start_vt();
            unsafe { KeRevertToUserAffinityThread() };
        }
    }
    
}

impl Drop for Vmm {
    fn drop(&mut self) {
        for (i,vcpu) in  self.vcpu.iter_mut().enumerate() {
            if vcpu.vmxon {
                unsafe { KeSetSystemAffinityThread(1 << i) };
                __vmx_off();
                unsafe { KeRevertToUserAffinityThread() };
            }
            vcpu.free_physical_memory();
        }
    }
}


pub fn protect_non_paged_memory(ptr: *mut c_void,size: u64,protection: u32) -> Result<(),& 'static str>{
    let mdl = unsafe { IoAllocateMdl(ptr,size as _,false as _,false as _,core::ptr::null_mut()) };
    if mdl.is_null() {
        return Err("IoAllocateMdl error");
    }

    unsafe { MmBuildMdlForNonPagedPool(mdl) };
    unsafe { (*mdl).MdlFlags |= MDL_MAPPED_TO_SYSTEM_VA as i16; } 
    let status = unsafe { MmProtectMdlSystemAddress(mdl, protection) };
    unsafe{ IoFreeMdl(mdl) };
    if !NT_SUCCESS(status) {
        return Err("MmProtectMdlSystemAddress error");
    }

    Ok(())
}


#[repr(C)]
#[derive(Copy,Clone)]
struct GDTENTRY64_ACCESS_RIGHTS_BYTES {
    flags1: u8,
    flags2: u8,
    flags3: u8,
    flags4: u8,
}

#[repr(C)]
#[derive(Copy,Clone)]
union GDTENTRY64_ACCESS_RIGHTS {
    access_rights: u32,
    bytes: GDTENTRY64_ACCESS_RIGHTS_BYTES,
    bits: u16,
} 

struct GdtEntry64 {
    selector: USHORT,
    limit: u32,
    access_rights: GDTENTRY64_ACCESS_RIGHTS,
    base: u64
}

#[repr(C)]
#[derive(Copy,Clone)]
struct KGDTENTRY64_U_BYTES{
    BaseMiddle: u8,
    Flags1: u8,
    Flags2: u8,
    BaseHigh: u8,
}

#[repr(C)]
#[derive(Copy,Clone)]
union KGDTENTRY64_U {
    bytes: KGDTENTRY64_U_BYTES,
    bits: u32,
}

#[repr(C)]
#[derive(Copy,Clone)]
struct KGDTENTRY64_A{
    limit_low: u16,
    base_low: u16,
    u: KGDTENTRY64_U,
    base_upper: u32,
    must_be_zero: u32,
}

#[repr(C)]
union KGDTENTRY64 {
    alignment: u64,
    dummy: KGDTENTRY64_A,
}


#[derive(Debug)]
enum VcpuVmxState {
    VmxStateOff,        // 未虚拟化
    VmxStateTransition, // 虚拟化中，还未恢复上下文
    VmxStateOn,         // 虚拟化成功
}

#[repr(C)]
#[derive(Debug)]
struct VmxVmcs {
    revision_id: u32, // 版本标识
    abort_indicator: u32,
    data: [u8; PAGE_SIZE - 2 * core::mem::size_of::<u32>()], // 4KB大小
}
const PAGE_SIZE: usize = 4096; // 假设页大小为4KB

#[allow(dead_code)]
#[derive(Debug, Default)]
struct VMXFeatures {
    secondary_controls: bool,      // Secondary controls are enabled
    true_msrs: bool,               // True VMX MSR values are supported
    ept: bool,                    // EPT supported by CPU
    vpid: bool,                   // VPID supported by CPU
    exec_only_ept: bool,            // EPT translation with execute-only access is supported
    inv_single_address: bool,       // IVVPID for single address
    vmfunc: bool,                 // VMFUNC is supported
}