pub mod check;
pub mod vmx;

pub mod ins {
    use core::{arch::asm, ffi::c_void};

    use wdk::println;

    #[derive(Debug)]
    pub enum VmxInstructionResult {
        VmxSuccess = 0,
        VmxFailValid = 1,
        VmxFailInvalid = 2,
    }

    impl From<u64> for VmxInstructionResult {
        fn from(value: u64) -> Self {
            match value {
                0 => VmxInstructionResult::VmxSuccess,
                1 => VmxInstructionResult::VmxFailValid,
                2 => VmxInstructionResult::VmxFailInvalid,
                _ => panic!("Invalid value for VmxInstructionResult"),
            }
        }
    }

    // success on rflags.cf = 0
    pub fn __vmx_on(value: *mut u64) -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmxon [rcx]",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") value,
                out("rax") result,
                options(nostack, nomem)
            );
        }
        VmxInstructionResult::from(result)
    }

    pub fn __vmx_off() -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmxoff",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                out("rax") result,
                options(nostack, nomem)
            );
        }

        VmxInstructionResult::from(result)
    }

    pub fn __vmx_vmclear(value: *mut u64) -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmclear [rcx]",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") value,
                out("rax") result,
                options(nostack, nomem)
            );
        }
        
        VmxInstructionResult::from(result)
    }

    pub fn __vmx_vmptrld(value: *mut u64) -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmptrld [rcx]",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") value,
                out("rax") result,
                options(nostack, nomem)
            );
        }

        VmxInstructionResult::from(result)
    }

    pub fn __invept(invept_type: u64,ept_ctx: *mut c_void) -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "invept rcx, [rdx]",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") invept_type,
                in("rdx") ept_ctx,
                out("rax") result,
                options(nostack, nomem)
            );
        }
        
        VmxInstructionResult::from(result)
    }

}