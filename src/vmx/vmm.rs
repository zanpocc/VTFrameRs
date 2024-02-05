use core::arch::global_asm;



// entrypoint
global_asm!(r#"
    .section .text

vmm_entry_point:
    int 3

"#);