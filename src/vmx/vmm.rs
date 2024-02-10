use core::arch::{asm, global_asm};

use wdk::println;
use wdk_sys::ntddk::KeGetCurrentIrql;

use crate::{cpu::cpu::ins::cpuidex, utils::utils::__debugbreak, vmx::{data::vmcs_encoding::{EXIT_QUALIFICATION, GUEST_LINEAR_ADDRESS, GUEST_PHYSICAL_ADDRESS, GUEST_RFLAGS, GUEST_RIP, GUEST_RSP, VM_EXIT_REASON}, ins::vmcs_read}, __GD};

use super::{data::{exit_reason::{EXIT_REASON_CPUID, EXIT_REASON_VMCALL}, vmcs_encoding::VM_EXIT_INSTRUCTION_LEN}, ins::{VmxInstructionResult, __vmx_off, __vmx_vmwrite}, vmx::{Vcpu, VcpuVmxState}};

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
	vcpu: *mut Vcpu,
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
        VM_CALL_UNLOAD => { 
            guest_state.exit_pending = true;
            return;
        }
        _ =>{
            println!("Unknown vmcall command");
        }
    }

    vmx_advance_eip(guest_state);
}

extern "C" fn vmx_exit_handler(context: &mut Context) -> u64 {

    let gd = unsafe { __GD.as_mut().unwrap() };

    let mut guest_state = GuestState{
        guest_regs: context,
        vcpu: gd.vmx_data.as_mut().unwrap().get_current_vcpu(),
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
        },
        EXIT_REASON_VMCALL => {
            println!("guest_rsp:{:X}",guest_state.guest_rsp);
            vm_exit_vmcall(&mut guest_state);
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

    println!("vmxoff success");
        
    let vcpu = unsafe { guest_state.vcpu.as_mut().unwrap() };
    vcpu.set_vmx_state(VcpuVmxState::VmxStateOff);

    return guest_state.guest_rip + ins_len;
}