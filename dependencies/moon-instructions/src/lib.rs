#![no_std]

use moon_struct::{cpuid::CPUID, inner::KDESCRIPTOR};

#[macro_export]
macro_rules! debugbreak {
    () => {
        unsafe {
            core::arch::asm!("int 3");
        }
    };
}

use core::arch::asm;

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

pub fn write_msr(msr: u32, value: u64) {
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

    result
}

pub fn write_cr3(value: u64) {
    unsafe {
        asm!(
            "mov cr3,rcx",
            in("rcx") value,
            options(nostack, nomem)
        );
    }
}

pub fn read_cr4() -> u64 {
    let mut result: u64;
    unsafe {
        asm!(
            "xor rax,rax",
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

pub fn read_cr8() -> u64 {
    let mut result: u64;
    unsafe {
        asm!(
            "xor rax,rax",
            "mov rax,cr8",
            out("rax") result,
            options(nostack, nomem)
        );
    }
    result
}

pub fn write_cr8(value: u64) {
    unsafe {
        asm!(
            "mov cr8,rcx",
            in("rcx") value,
            options(nostack, nomem)
        );
    }
}

pub fn lgdt(addr: &KDESCRIPTOR) {
    unsafe {
        asm!(
            "lgdt [rax+6]",
            in("rax") addr,
            options(nostack, nomem)
        );
    }
}

pub fn lidt(addr: &KDESCRIPTOR) {
    unsafe {
        asm!(
            "lidt [rax+6]",
            in("rax") addr,
            options(nostack, nomem)
        );
    }
}

pub fn stosq(destination: *mut u64, value: u64, count: u64) {
    unsafe {
        asm!(
            "rep stosq",
            in("rdi") destination,
            in("rax") value,
            in("rcx") count,
            options(nostack, nomem)
        );
    }
}

// find bit value eq 1 in binary range mask
pub fn bit_scan_forward64(index: *mut u32, mask: u64) {
    unsafe {
        asm!(
            "bsf rax,rax",
            "mov [rcx],rax",
            in("rax") mask,
            in("rcx") index,
            options(nostack, nomem)
        );
    }
}

pub fn cpuidex(eax: u32, ecx: u32) -> CPUID {
    let mut result = CPUID::default();

    // !!! do not modify regist ebx anyway
    unsafe {
        asm!(
            "push rbx",
            "push rdx",

            "cpuid",
            "mov [rdi],eax",
            "mov [rdi+4],ebx",
            "mov [rdi+8],ecx",
            "mov [rdi+0xc],edx",

            "pop rdx",
            "pop rbx",

            in("ecx") ecx,
            in("eax") eax,
            in("rdi") &mut result,
            options(nostack, nomem)
        );
    }

    result
}
