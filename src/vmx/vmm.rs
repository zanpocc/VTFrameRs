use core::arch::{asm, global_asm};

use wdk::println;
use wdk_sys::{ntddk::KeGetCurrentIrql, LARGE_INTEGER};

use crate::{cpu::cpu::{ins::{cpuidex, read_msr, write_msr}, stru::{msr::{self, ia32_feature_control_msr::{ENABLE_VMXON, LOCK_MASK}}, msr_index::{MSR_FS_BASE, MSR_GS_BASE, MSR_IA32_DEBUGCTL, MSR_IA32_FEATURE_CONTROL, MSR_IA32_VMX_BASIC, MSR_IA32_VMX_VMFUNC, MSR_RESERVED_MAX, MSR_RESERVED_MIN, MSR_UNKNOWN, MSR_UNKNOWN2, VMWARE_MSR, VMWARE_MSR2, VMWARE_MSR3, VMWARE_MSR4}}}, utils::utils::__debugbreak, vmx::{data::vmcs_encoding::{EXIT_QUALIFICATION, GUEST_LINEAR_ADDRESS, GUEST_PHYSICAL_ADDRESS, GUEST_RFLAGS, GUEST_RIP, GUEST_RSP, VM_EXIT_REASON}, ins::vmcs_read}, __GD};

use super::{data::{exit_reason::{EXIT_REASON_CPUID, EXIT_REASON_CR_ACCESS, EXIT_REASON_MSR_READ, EXIT_REASON_MSR_WRITE, EXIT_REASON_VMCALL}, mov_cr_qualification, vm_call::VM_CALL_CLOSE_VT, vmcs_encoding::{self, CR0_READ_SHADOW, CR4_READ_SHADOW, GUEST_CR0, GUEST_CR3, GUEST_CR4, GUEST_FS_BASE, GUEST_GS_BASE, GUEST_IA32_DEBUGCTL, VM_EXIT_INSTRUCTION_LEN}, TYPE_MOV_FROM_CR, TYPE_MOV_TO_CR}, ins::{VmxInstructionResult, __vmx_off, __vmx_vmwrite}};

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

fn vm_exit_cpuid(guest_state: &mut GuestState) {

    let cpuinfo = cpuidex(unsafe { guest_state.guest_regs.as_ref().unwrap().rax as _ },unsafe { guest_state.guest_regs.as_ref().unwrap().rcx as _ });

    unsafe{
        (*guest_state.guest_regs).rax = cpuinfo.eax as _;
        (*guest_state.guest_regs).rbx = cpuinfo.ebx as _;
        (*guest_state.guest_regs).rcx = cpuinfo.ecx as _;
        (*guest_state.guest_regs).rdx = cpuinfo.edx as _;
    }

    vmx_advance_eip(guest_state);
}

fn vm_exit_vmcall(guest_state: &mut GuestState) {
    //获取第一个参数，功能类型编号
    match unsafe { guest_state.guest_regs.as_ref().unwrap().rcx } & 0xFFFF {
        VM_CALL_CLOSE_VT => { 
            guest_state.exit_pending = true;
            return;
        }
        _ =>{
            println!("Unknown vmcall command");
        }
    }

    vmx_advance_eip(guest_state);
}

fn vm_exit_msr_read(guest_state: &mut GuestState) {

    let mut msr_value = LARGE_INTEGER::default();

    let ecx: u32 = unsafe { guest_state.guest_regs.as_mut().unwrap().rcx } as u32;
    println!("msr read:{}",ecx);

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
                msr_value.QuadPart |= ENABLE_VMXON as i64;
                msr_value.QuadPart |= LOCK_MASK as i64;
            }
        }
        MSR_IA32_VMX_BASIC..=MSR_IA32_VMX_VMFUNC => {
            // todo vmx msr
            println!("vmx msr:{}",ecx);
        }
        VMWARE_MSR2 => {
            msr_value.QuadPart = read_msr(ecx as _) as _;
        }
        _ => {
            if ecx >= MSR_RESERVED_MIN && ecx <= MSR_RESERVED_MAX {
                // todo inject event
            } else if ecx == MSR_UNKNOWN || ecx == MSR_UNKNOWN2 {
                // todo inject event
            }

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

    println!("msr write:{}",ecx);

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
        MSR_IA32_VMX_BASIC..=MSR_IA32_VMX_VMFUNC => {
            // todo vmx msr
            println!("vmx msr:{}",ecx);
        }   
        VMWARE_MSR | VMWARE_MSR3 | VMWARE_MSR4 => {
            write_msr(ecx as _, unsafe{ msr_value.QuadPart } as _);
        }
        _ => {
            if ecx >= MSR_RESERVED_MIN && ecx <= MSR_RESERVED_MAX {
                // todo inject event
            } else if ecx == MSR_UNKNOWN || ecx == MSR_UNKNOWN2 {
                // todo inject event
            }

            write_msr(ecx, unsafe{ msr_value.QuadPart } as _);
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
            println!("unknown cr index");
            panic!();
        }
    }
}

fn vm_exit_cr_access(guest_state: &mut GuestState) {
    let data: u64 = guest_state.exit_qualification; // MOV_CR_QUALIFICATION
    let reg: &mut u64 = get_cr_select_register((data & mov_cr_qualification::REGISTER_MASK as u64) as u32,guest_state);
    let access_type = (data & mov_cr_qualification::ACCESS_TYPE_MASK as u64) as u32;

    println!("cr access reg={},at={}",reg,access_type);

    match access_type {
        TYPE_MOV_TO_CR => {
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
                    println!("unknown cr write");
                }
            }
            
        }
        TYPE_MOV_FROM_CR => {
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
                    println!("unknown cr read");
                }
            }
        }
        _ => {
            println!("error cr access type");
        }
    }

}

unsafe extern "C" fn vmx_exit_handler(context: &mut Context) -> u64 {

    let mut guest_state = GuestState{
        guest_regs: context,
        // vcpu: gd.vmx_data.as_mut().unwrap().get_current_vcpu(),
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

    match guest_state.exit_reason {
        EXIT_REASON_CPUID => {
            vm_exit_cpuid(&mut guest_state);
        }
        EXIT_REASON_VMCALL => {
            vm_exit_vmcall(&mut guest_state);
        }
        EXIT_REASON_MSR_READ => {
            vm_exit_msr_read(&mut guest_state);
        }
        EXIT_REASON_MSR_WRITE => {
            vm_exit_msr_write(&mut guest_state);
        }
        EXIT_REASON_CR_ACCESS => {
            vm_exit_cr_access(&mut guest_state);
        }
        _ => {
            println!("exit_reason:{}",guest_state.exit_reason);

            unsafe{ 
                asm!{
                    "int 3"
                };
            }
        }
    }

    if !guest_state.exit_pending{
        return 0;
    }

    let ins_len = vmcs_read(VM_EXIT_INSTRUCTION_LEN);
    unsafe{ guest_state.guest_regs.as_mut().unwrap().rsp = guest_state.guest_rsp };

    if __vmx_off() != VmxInstructionResult::VmxSuccess {
        println!("vmx_off execute error");
        __debugbreak();
    }

    return guest_state.guest_rip + ins_len;
}