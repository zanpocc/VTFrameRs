use core::{ffi::c_void, i128::MAX, mem::size_of, ptr::write_bytes};

use alloc::vec::Vec;
use wdk::println;
use wdk_sys::{ntddk::{IoAllocateMdl, IoFreeMdl, KeGetCurrentProcessorNumberEx, KeQueryActiveProcessorCount, KeRevertToUserAffinityThread, KeSetSystemAffinityThread, MmAllocateContiguousMemory, MmBuildMdlForNonPagedPool, MmFreeContiguousMemory, MmGetPhysicalAddress, MmProtectMdlSystemAddress, RtlCaptureContext}, CONTEXT, KERNEL_STACK_SIZE, MDL_MAPPED_TO_SYSTEM_VA, NT_SUCCESS, PAGE_READWRITE, PHYSICAL_ADDRESS, _LARGE_INTEGER};

use crate::{cpu::cpu::{ins::{read_cr0, read_cr4, read_msr, write_cr0, write_cr4}, stru::msr_index::{MSR_IA32_VMX_BASIC, MSR_IA32_VMX_CR0_FIXED0, MSR_IA32_VMX_CR0_FIXED1, MSR_IA32_VMX_CR4_FIXED0, MSR_IA32_VMX_CR4_FIXED1}}, inner::{data_struct::KPROCESSOR_STATE, KeSaveStateForHibernate}, utils::utils::create_end_mask, vmx::ins::{__vmx_on, __vmx_vmclear, __vmx_vmptrld}};

use super::ins::{VmxInstructionResult, __vmx_off};

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
        host_state.SpecialRegisters.Cr0 &= cr0_fixed1 & create_end_mask(31); // lowpart
        host_state.SpecialRegisters.Cr0 |= cr0_fixed0 & create_end_mask(31); // lowpart
        host_state.SpecialRegisters.Cr4 &= cr4_fixed1 & create_end_mask(31); // lowpart
        host_state.SpecialRegisters.Cr4 |= cr4_fixed0 & create_end_mask(31); // lowpart

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

    fn subvert_cpu(&mut self) {
        let vcpu = self;
        // force cast
        let phys:PHYSICAL_ADDRESS = unsafe { core::mem::transmute(&u64::MAX) };

        // need free it youself on drop fuction
        let vmxon = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _,phys) };
        let vmcs = unsafe { MmAllocateContiguousMemory(PAGE_SIZE as _,phys) };
        let vmm_stack = unsafe { MmAllocateContiguousMemory(KERNEL_STACK_SIZE as _,phys) };

        // allocate fault
        if vmxon.is_null() || vmcs.is_null() || vmm_stack.is_null() {
            return;
        }

        vcpu.vm_resources.vmxon = vmxon as _;
        vcpu.vm_resources.vmcs = vmcs as _;
        vcpu.vm_resources.vmm_stack = vmm_stack;

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

        // zero memory
        unsafe{
            vmxon.write_bytes(0, size_of::<VmxVmcs>());
            vmcs.write_bytes(0, size_of::<VmxVmcs>());
            vmm_stack.write_bytes(0, KERNEL_STACK_SIZE as _);
        }

        // enter vmx root
        match vcpu.enter_vmx_root_mode() {
            Ok(_) =>{}
            Err(e) => {
                println!("{}",e);
                return;
            }
        }

        println!("already enter vmx root mode");

        // set vmcs todo

    }

    fn start_vt(&mut self) {
        let host_state = &mut self.host_state;
        let host_state_ptr: *mut KPROCESSOR_STATE = host_state;
        let context_frame_ptr: *mut CONTEXT = &mut host_state.context_frame;

        println!("before rip:{:X}",self.host_state.context_frame.Rip);
        unsafe{ KeSaveStateForHibernate(host_state_ptr); }
        println!("before rip:{:X}",self.host_state.context_frame.Rip);
        // important!!!!
        // continue on next code after execute vmx_on instruction
        unsafe { RtlCaptureContext(context_frame_ptr); }
        println!("before rip:{:X}",self.host_state.context_frame.Rip);

        match self.vcpu_vmx_state {
            VcpuVmxState::VmxStateOff => {
                // begin start vt
                self.subvert_cpu();
            }
            VcpuVmxState::VmxStateTransition => {
                // vmlauch execute successed 
                // todo
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

struct VmcsResources {
    vmxon: *mut VmxVmcs,
    vmcs: *mut VmxVmcs,
    vmm_stack: *mut c_void,
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