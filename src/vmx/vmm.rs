use core::arch::global_asm;

use moon_driver_utils::{bitfield::set_bits_value32, page_align};
use moon_instructions::{cpuidex, debugbreak, lgdt, lidt, read_msr, write_cr3, write_msr};
use moon_log::{error, warn};
use moon_struct::{inner::KDESCRIPTOR, msr::{self, ia32_feature_control_msr::{MSR_IA32_FEATURE_CONTROL_LOCK, MSR_IA32_FEATURE_CONTROL_VMXON}, msr_index::{MSR_FS_BASE, MSR_GS_BASE, MSR_IA32_DEBUGCTL, MSR_IA32_FEATURE_CONTROL}}};
use wdk_sys::{ntddk::KeGetCurrentIrql, LARGE_INTEGER};

use crate::{utils::utils::virtual_address_to_physical_address, vmx::{data::{vmcs_encoding::{EXIT_QUALIFICATION, GUEST_LINEAR_ADDRESS, GUEST_PHYSICAL_ADDRESS, GUEST_RFLAGS, GUEST_RIP, GUEST_RSP, VM_EXIT_REASON}, TYPE_CR_READ, TYPE_CR_WRITE}, ins::vmcs_read}, __GD};

use super::{data::{interrupt_inject_info::{TYPE_LEN, TYPE_START, VALID, VECTOR_LEN, VECTOR_START}, interrupt_type::INTERRUPT_HARDWARE_EXCEPTION, mov_cr_qualification, page_hook_attrib::{PAGE_ATTRIBE_EXECUTE, PAGE_ATTRIBE_READ, PAGE_ATTRIBE_WRITE}, vector_exception::VECTOR_INVALID_OPCODE_EXCEPTION, vm_call::{self, INVEPT_ALL_CONTEXT, INVEPT_SINGLE_CONTEXT}, vmcs_encoding::{CR0_READ_SHADOW, CR4_READ_SHADOW, GUEST_CR0, GUEST_CR3, GUEST_CR4, GUEST_FS_BASE, GUEST_GDTR_BASE, GUEST_GDTR_LIMIT, GUEST_GS_BASE, GUEST_IA32_DEBUGCTL, GUEST_IDTR_BASE, GUEST_IDTR_LIMIT, VM_ENTRY_INSTRUCTION_LEN, VM_ENTRY_INTR_INFO_FIELD, VM_EXIT_INSTRUCTION_LEN}}, ept::ept::InveptDescriptor, ins::{VmxInstructionResult, __invept, __vmx_off, __vmx_vmwrite}};


global_asm!(r#"
.section .text

.macro pushaq
    push    -1      // rsp
    push    rax
    push    rcx
    push    rdx
    push    rbx
    
    push    rbp
    push    rsi
    push    rdi
    push    r8
    push    r9
    push    r10
    push    r11
    push    r12
    push    r13
    push    r14
    push    r15
.endm


.macro popaq
    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     r11
    pop     r10
    pop     r9
    pop     r8
    pop     rdi
    pop     rsi
    pop     rbp
       
    pop     rbx
    pop     rdx
    pop     rcx
    pop     rax
    add     rsp, 8  // rsp
.endm

.macro popaq_exit
    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     r11
    pop     r10
    pop     r9
    pop     r8
    pop     rdi
    pop     rsi
    pop     rbp
       
    pop     rbx
    pop     rdx
    pop     rcx
.endm

vmm_entry_point:
    pushaq
    mov rcx, rsp

    sub rsp, 0x60
    movaps [rsp +  0x0], xmm0
    movaps [rsp + 0x10], xmm1
    movaps [rsp + 0x20], xmm2
    movaps [rsp + 0x30], xmm3
    movaps [rsp + 0x40], xmm4
    movaps [rsp + 0x50], xmm5

    sub rsp, 0x20
    call {}
    add rsp, 0x20

    movaps xmm0, [rsp + 0x0]
    movaps xmm1, [rsp + 0x10]
    movaps xmm2, [rsp + 0x20]
    movaps xmm3, [rsp + 0x30]
    movaps xmm4, [rsp + 0x40]
    movaps xmm5, [rsp + 0x50]
    add rsp, 0x60

    cmp rax, 0
    jne exit_branch
    
    popaq
    vmresume
    int 3

exit_branch:
    popaq_exit
    add rsp, 8      // rax
    pop rsp         // rsp
    jmp rax         // guest_rip

    int 3
"#,sym vmx_exit_handler);


#[repr(C)]
#[derive(Debug)]
struct Context {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    rsp: u64,
}


#[allow(unused)]
struct GuestState
{
	guest_regs: *mut Context,
	// vcpu: *mut Vcpu,
    guest_rip: u64,
    guest_rsp: u64,
    guest_rflags: u64,
    linear_address: u64,
    physical_address: u64,
    guest_irql: u8,
    exit_reason: u16,
    exit_qualification: u64,
    exit_pending: bool,
} 

fn vmx_advance_eip(guest_state: &mut GuestState) {
    guest_state.guest_rip += vmcs_read(VM_EXIT_INSTRUCTION_LEN);
    __vmx_vmwrite(GUEST_RIP, guest_state.guest_rip);
}

fn vmx_inject_event(interrupt_type: u32,vector_exception: u8,write_length: u32) {
    let mut inject_event:u32 = 0;

    inject_event = set_bits_value32(inject_event,VECTOR_START,VECTOR_LEN,vector_exception as _);
    inject_event = set_bits_value32(inject_event,TYPE_START,TYPE_LEN,interrupt_type as _);
    inject_event |= VALID;

    __vmx_vmwrite(VM_ENTRY_INTR_INFO_FIELD, inject_event as _);
    if write_length > 0 {
        __vmx_vmwrite(VM_ENTRY_INSTRUCTION_LEN, write_length as _);
    }
}

fn vm_exit_cpuid(guest_state: &mut GuestState) {

    let cpuinfo = cpuidex(unsafe { guest_state.guest_regs.as_ref().unwrap().rax as _ },unsafe { guest_state.guest_regs.as_ref().unwrap().rcx as _ });

    unsafe{
        let reg = guest_state.guest_regs.as_mut().unwrap();
        reg.rax = cpuinfo.eax as _;
        reg.rbx = cpuinfo.ebx as _;
        reg.rcx = cpuinfo.ecx as _;
        reg.rdx = cpuinfo.edx as _;
    }

    vmx_advance_eip(guest_state);
}

fn invept_single(eptp: u64) {
    let mut descriptor = InveptDescriptor::default();
    descriptor.ept_pointer = eptp;
    descriptor.reserved = 0;
    
    __invept(INVEPT_SINGLE_CONTEXT, &mut descriptor as *mut _ as _);
}

fn invept_all() {
    __invept(INVEPT_ALL_CONTEXT, core::ptr::null_mut());
}

fn ept_perform_page_hook(target_address: *mut u8, _hook_function_address: *mut u8, page_attribe: u64) -> Result<(),& 'static str>{
    let _r = page_attribe & PAGE_ATTRIBE_READ;
    let _w = page_attribe & PAGE_ATTRIBE_WRITE;
    let _e = page_attribe & PAGE_ATTRIBE_EXECUTE;

    let virtual_target = page_align!(target_address);

    let physical_target = virtual_address_to_physical_address(virtual_target as _);
    if physical_target == 0 {
        return Err("Target address could not be mapped to physical memory");
    }

    

    Ok(())
}

fn vm_exit_vmcall(guest_state: &mut GuestState) {
    unsafe {
        let reg = guest_state.guest_regs.as_mut().unwrap();
        let option_param1 =reg.rdx;
        let option_param2 =reg.r8 ;
        let option_param3 =reg.r9 ;

        //获取第一个参数，功能类型编号
        match reg.rcx & 0xFFFFFFFF {
            vm_call::EXIT_VT => { 
                guest_state.exit_pending = true;
                return;
            }
            vm_call::PAGE_HOOK => {
                let _ = ept_perform_page_hook(option_param1 as _,option_param2 as _,option_param3);
            }
            vm_call::INVEPT_SINGLE_CONTEXT => {
                invept_single(__GD.as_mut().unwrap().vmm.as_mut().unwrap().ept_state.as_mut().unwrap().get_ept_pointer());
            }
            vm_call::INVEPT_ALL_CONTEXT => {
                invept_all();
            }
            _ =>{
                error!("Unknown vmcall command");
            }
        }
    }

    vmx_advance_eip(guest_state);
}

fn vm_exit_msr_read(guest_state: &mut GuestState) {

    let mut msr_value = LARGE_INTEGER::default();

    let ecx: u32 = unsafe { guest_state.guest_regs.as_mut().unwrap().rcx } as u32;

    match ecx {
        MSR_GS_BASE => {
            msr_value.QuadPart = vmcs_read(GUEST_GS_BASE) as _;
        }
        MSR_FS_BASE => {
            msr_value.QuadPart = vmcs_read(GUEST_FS_BASE) as _;
        }
        MSR_IA32_DEBUGCTL => {
            msr_value.QuadPart = vmcs_read(GUEST_IA32_DEBUGCTL) as _;
        }
        MSR_IA32_FEATURE_CONTROL => {
            msr_value.QuadPart = vmcs_read(ecx as _) as _;
            unsafe{
                msr_value.QuadPart |= MSR_IA32_FEATURE_CONTROL_VMXON as i64;
                msr_value.QuadPart |= MSR_IA32_FEATURE_CONTROL_LOCK as i64;
            }
        }
        msr::msr_index::MSR_IA32_VMX_BASIC..=msr::msr_index::MSR_IA32_VMX_VMFUNC => {
            // todo vmx msr
            warn!("vmx msr:{:X}",ecx);
        }
        msr::msr_index::MSR_CRASH_CTL => {
            msr_value.QuadPart = read_msr(ecx as _) as _;
        }
        _ => {
            // if ecx >= msr::msr_index::MSR_RESERVED_MIN && ecx <= msr::msr_index::MSR_RESERVED_MAX {
            //     // todo inject event
            // } else if ecx == msr::msr_index::MSR_UNKNOWN || ecx == msr::msr_index::MSR_UNKNOWN2 {
            //     // todo inject event
            // }

            msr_value.QuadPart = read_msr(ecx) as _;
        }
    }

    unsafe{
        guest_state.guest_regs.as_mut().unwrap().rax = msr_value.u.LowPart as _;
        guest_state.guest_regs.as_mut().unwrap().rdx = msr_value.u.HighPart as _;
    }
     
     vmx_advance_eip(guest_state);

}

fn vm_exit_msr_write(guest_state: &mut GuestState) {
    let mut msr_value = LARGE_INTEGER::default();
    let ecx: u32 = unsafe { guest_state.guest_regs.as_mut().unwrap().rcx } as u32;

    unsafe{
        msr_value.u.LowPart = guest_state.guest_regs.as_ref().unwrap().rax as _;
        msr_value.u.HighPart = guest_state.guest_regs.as_ref().unwrap().rdx as _;
    }

    match ecx {
        MSR_GS_BASE => {
            write_msr(GUEST_GS_BASE as _, unsafe{msr_value.QuadPart} as _);
        }
        MSR_FS_BASE => {
            write_msr(GUEST_FS_BASE as _, unsafe{msr_value.QuadPart} as _);
        }
        MSR_IA32_DEBUGCTL => {
            unsafe{
                __vmx_vmwrite(GUEST_IA32_DEBUGCTL, msr_value.QuadPart as _);
                write_msr(MSR_IA32_DEBUGCTL, msr_value.QuadPart as _);
            }
        }
        msr::msr_index::MSR_IA32_VMX_BASIC..=msr::msr_index::MSR_IA32_VMX_VMFUNC => {
            // todo vmx msr
            warn!("vmx msr:{:X}",ecx);
        }
        // VMware
        msr::msr_index::MSR_STIMER0_CONFIG | msr::msr_index::MSR_STIMER0_COUNT | msr::msr_index::MSR_CRASH_P0 => {
            write_msr(ecx as _, unsafe{ msr_value.QuadPart } as _);
        }
        _ => {
            if unsafe { __GD.as_mut().unwrap().vmm.as_mut().unwrap().vmx_features.in_vmware } {
                write_msr(ecx, unsafe{ msr_value.QuadPart } as _);
            } else {
                if ecx >= msr::msr_index::MSR_RESERVED_MIN && ecx <= msr::msr_index::MSR_RESERVED_MAX {
                    warn!("MSR_RESERVED:{:X}",ecx);
                    vmx_inject_event(INTERRUPT_HARDWARE_EXCEPTION,VECTOR_INVALID_OPCODE_EXCEPTION,0);
                    return;
                } else if ecx == msr::msr_index::MSR_UNKNOWN || ecx == msr::msr_index::MSR_UNKNOWN2 {
                    warn!("MSR_UNKNOWN:{:X}",ecx);
                    vmx_inject_event(INTERRUPT_HARDWARE_EXCEPTION,VECTOR_INVALID_OPCODE_EXCEPTION,0);
                    return;
                }
            }
        }
    }

    vmx_advance_eip(guest_state);

}


fn get_cr_select_register(index: u32,guest_state: &mut GuestState) -> &mut u64 {
    return match index {
        0 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rax }
        1 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rcx }
        2 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rdx }
        3 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rbx }
        4 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rsp }
        5 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rbp }
        6 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rsi }
        7 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().rdi }
        8 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r8 }
        9 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r9 }
        10 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r10 }
        11 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r11 }
        12 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r12 }
        13 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r13 }
        14 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r14 }
        15 => unsafe { &mut guest_state.guest_regs.as_mut().unwrap().r15 }
        _ => {
            error!("unknown cr index");
            panic!();
        }
    }
}

fn vm_exit_cr_access(guest_state: &mut GuestState) {
    let data = guest_state.exit_qualification; // MOV_CR_QUALIFICATION
    let reg = get_cr_select_register((data & mov_cr_qualification::REGISTER_MASK as u64) as u32,guest_state);
    let access_type = (data & mov_cr_qualification::ACCESS_TYPE_MASK as u64) as u32;

    match access_type {
        TYPE_CR_WRITE => {
            let control_register = data & mov_cr_qualification::CONTROL_REGISTER_MASK as u64;
            match control_register {
                0 => {
                    __vmx_vmwrite(GUEST_CR0, *reg);
                    __vmx_vmwrite(CR0_READ_SHADOW, *reg);
                }
                3 => {
                    __vmx_vmwrite(GUEST_CR3, *reg & !(1u64 << 63));
                }
                4 => {
                    __vmx_vmwrite(GUEST_CR4, *reg);
                    __vmx_vmwrite(CR4_READ_SHADOW, *reg);
                }
                _ => {
                    error!("unknown cr write");
                }
            }
        }
        TYPE_CR_READ => {
            let control_register = data & mov_cr_qualification::CONTROL_REGISTER_MASK as u64;
            match control_register {
                0 => {
                    *reg = vmcs_read(GUEST_CR0);
                }
                3 => {
                    *reg = vmcs_read(GUEST_CR3);
                }
                4 => {
                    *reg = vmcs_read(GUEST_CR4);
                }
                _ => {
                    error!("unknown cr read");
                }
            }
        }
        _ => {
            error!("error cr access type");
        }
    }

}

fn vm_exit_vmop(_guest_state: &mut GuestState) {
    vmx_inject_event(INTERRUPT_HARDWARE_EXCEPTION, VECTOR_INVALID_OPCODE_EXCEPTION, 0 );
}

fn vm_exit_unknown(guest_state: &mut GuestState) {
    warn!("vm_exit_unknown,resaoin:{},rip:{:X}",guest_state.exit_reason,guest_state.guest_rip);
    debugbreak!();
}

fn vm_exit_ept_misconfig(_guest_state: &mut GuestState) {
    warn!("todo vm_exit_ept_misconfig");
    debugbreak!();
}

fn vm_exit_ept_violation(_guest_state: &mut GuestState) {
    warn!("todo vm_exit_ept_violation");
    debugbreak!();
}

type ExitHandler = fn(guest_state: &mut GuestState);
static EXIT_HANDLER:[ExitHandler;65] = [
    vm_exit_unknown,      // 00 EXIT_REASON_EXCEPTION_NMI
    vm_exit_unknown,      // 01 EXIT_REASON_EXTERNAL_INTERRUPT
    vm_exit_unknown,      // 02 EXIT_REASON_TRIPLE_FAULT
    vm_exit_unknown,      // 03 EXIT_REASON_INIT
    vm_exit_unknown,      // 04 EXIT_REASON_SIPI
    vm_exit_unknown,      // 05 EXIT_REASON_IO_SMI
    vm_exit_unknown,      // 06 EXIT_REASON_OTHER_SMI
    vm_exit_unknown,      // 07 EXIT_REASON_PENDING_INTERRUPT
    vm_exit_unknown,      // 08 EXIT_REASON_NMI_WINDOW
    vm_exit_unknown,      // 09 EXIT_REASON_TASK_SWITCH
    vm_exit_cpuid,        // 10 EXIT_REASON_CPUID
    vm_exit_unknown,      // 11 EXIT_REASON_GETSEC
    vm_exit_unknown,      // 12 EXIT_REASON_HLT
    vm_exit_unknown,      // 13 EXIT_REASON_INVD
    vm_exit_unknown,      // 14 EXIT_REASON_INVLPG
    vm_exit_unknown,      // 15 EXIT_REASON_RDPMC
    vm_exit_unknown,      // 16 EXIT_REASON_RDTSC
    vm_exit_unknown,      // 17 EXIT_REASON_RSM
    vm_exit_vmcall,       // 18 EXIT_REASON_VMCALL
    vm_exit_vmop,         // 19 EXIT_REASON_VMCLEAR
    vm_exit_vmop,         // 20 EXIT_REASON_VMLAUNCH
    vm_exit_vmop,         // 21 EXIT_REASON_VMPTRLD
    vm_exit_vmop,         // 22 EXIT_REASON_VMPTRST
    vm_exit_vmop,         // 23 EXIT_REASON_VMREAD
    vm_exit_vmop,         // 24 EXIT_REASON_VMRESUME
    vm_exit_vmop,         // 25 EXIT_REASON_VMWRITE
    vm_exit_vmop,         // 26 EXIT_REASON_VMXOFF
    vm_exit_vmop,         // 27 EXIT_REASON_VMXON
    vm_exit_cr_access,    // 28 EXIT_REASON_CR_ACCESS
    vm_exit_unknown,      // 29 EXIT_REASON_DR_ACCESS
    vm_exit_unknown,      // 30 EXIT_REASON_IO_INSTRUCTION
    vm_exit_msr_read,     // 31 EXIT_REASON_MSR_READ
    vm_exit_msr_write,    // 32 EXIT_REASON_MSR_WRITE
    vm_exit_unknown,      // 33 EXIT_REASON_INVALID_GUEST_STATE
    vm_exit_unknown,      // 34 EXIT_REASON_MSR_LOADING
    vm_exit_unknown,      // 35 EXIT_REASON_RESERVED_35
    vm_exit_unknown,      // 36 EXIT_REASON_MWAIT_INSTRUCTION
    vm_exit_unknown,      // 37 EXIT_REASOM_MTF
    vm_exit_unknown,      // 38 EXIT_REASON_RESERVED_38
    vm_exit_unknown,      // 39 EXIT_REASON_MONITOR_INSTRUCTION
    vm_exit_unknown,      // 40 EXIT_REASON_PAUSE_INSTRUCTION
    vm_exit_unknown,      // 41 EXIT_REASON_MACHINE_CHECK
    vm_exit_unknown,      // 42 EXIT_REASON_RESERVED_42
    vm_exit_unknown,      // 43 EXIT_REASON_TPR_BELOW_THRESHOLD
    vm_exit_unknown,      // 44 EXIT_REASON_APIC_ACCESS
    vm_exit_unknown,      // 45 EXIT_REASON_VIRTUALIZED_EIO
    vm_exit_unknown,      // 46 EXIT_REASON_XDTR_ACCESS
    vm_exit_unknown,      // 47 EXIT_REASON_TR_ACCESS
    vm_exit_ept_violation,// 48 EXIT_REASON_EPT_VIOLATION
    vm_exit_ept_misconfig,// 49 EXIT_REASON_EPT_MISCONFIG
    vm_exit_vmop,         // 50 EXIT_REASON_INVEPT
    vm_exit_unknown,      // 51 EXIT_REASON_RDTSCP
    vm_exit_unknown,      // 52 EXIT_REASON_PREEMPT_TIMER
    vm_exit_vmop,         // 53 EXIT_REASON_INVVPID
    vm_exit_unknown,      // 54 EXIT_REASON_WBINVD
    vm_exit_unknown,      // 55 EXIT_REASON_XSETBV
    vm_exit_unknown,      // 56 EXIT_REASON_APIC_WRITE
    vm_exit_unknown,      // 57 EXIT_REASON_RDRAND
    vm_exit_unknown,      // 58 EXIT_REASON_INVPCID
    vm_exit_unknown,      // 59 EXIT_REASON_VMFUNC
    vm_exit_unknown,      // 60 EXIT_REASON_RESERVED_60
    vm_exit_unknown,      // 61 EXIT_REASON_RDSEED
    vm_exit_unknown,      // 62 EXIT_REASON_RESERVED_62
    vm_exit_unknown,      // 63 EXIT_REASON_XSAVES
    vm_exit_unknown       // 64 EXIT_REASON_XRSTORS
];

unsafe extern "C" fn vmx_exit_handler(context: &mut Context) -> u64 {
    let mut guest_state = GuestState{
        guest_regs: context,
        // vcpu: __GD.as_mut().unwrap().vmx_data.as_mut().unwrap().get_current_vcpu(),
        guest_rip: vmcs_read(GUEST_RIP),
        guest_rsp: vmcs_read(GUEST_RSP),
        guest_rflags: vmcs_read(GUEST_RFLAGS),
        linear_address: vmcs_read(GUEST_LINEAR_ADDRESS),
        physical_address: vmcs_read(GUEST_PHYSICAL_ADDRESS),
        guest_irql: unsafe { KeGetCurrentIrql() } as _,
        exit_reason: vmcs_read(VM_EXIT_REASON) as u16,
        exit_qualification: vmcs_read(EXIT_QUALIFICATION),
        exit_pending: false,
    };

    EXIT_HANDLER[guest_state.exit_reason as usize](&mut guest_state);

    // normal situation
    if !guest_state.exit_pending{
        return 0;
    }

    let ins_len = vmcs_read(VM_EXIT_INSTRUCTION_LEN);
    unsafe{ guest_state.guest_regs.as_mut().unwrap().rsp = guest_state.guest_rsp };
    
    // gdt,idt
    let mut gdtr:KDESCRIPTOR = KDESCRIPTOR::default();
    gdtr.Base = vmcs_read(GUEST_GDTR_BASE);
    gdtr.Limit = vmcs_read(GUEST_GDTR_LIMIT) as _;
    let mut idtr:KDESCRIPTOR = KDESCRIPTOR::default();
    idtr.Base = vmcs_read(GUEST_IDTR_BASE);
    idtr.Limit = vmcs_read(GUEST_IDTR_LIMIT) as _;

    lgdt(&gdtr);
    lidt(&idtr);
    write_cr3(vmcs_read(GUEST_CR3));

    if __vmx_off() != VmxInstructionResult::VmxSuccess {
        error!("vmx_off execute error");
        debugbreak!();
    }

    return guest_state.guest_rip + ins_len;
}