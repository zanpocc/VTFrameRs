use core::arch::asm;

use wdk_sys::{
    ntddk::{KeLowerIrql, KfRaiseIrql},
    DISPATCH_LEVEL, KIRQL,
};

pub fn wpoff() -> KIRQL {
    unsafe {
        let irql = KfRaiseIrql(DISPATCH_LEVEL as _);
        asm! {
            "push rax",
            "mov rax,cr0",
            "and rax,0xfffffffffffeffff",
            "mov cr0,rax",
            "pop rax",

            "cli",
        };
        irql
    }
}

pub fn wpon(irql: KIRQL) {
    unsafe {
        asm! {
            "mov rax,cr0",
            "or rax,0x10000",
            "sti",
            "mov cr0,rax",
        };
        KeLowerIrql(irql);
    }
}
