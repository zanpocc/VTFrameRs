pub mod check;
pub mod vmx;
pub mod data;
pub mod vmm;

pub mod ins {
    use core::{arch::asm, ffi::c_void};

    use wdk::println;

    use super::data::vmcs_encoding::VM_INSTRUCTION_ERROR;

    #[derive(Debug,PartialEq, Eq)]
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

    pub fn __vmx_vmlaunch() -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmlaunch",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                out("rax") result,
                options(nostack, nomem)
            );
        }

        VmxInstructionResult::from(result)
    }

    pub fn __vmx_vmwrite(field: u64,value: u64) -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmwrite rcx,rdx",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") field,
                in("rdx") value,
                out("rax") result,
                options(nostack, nomem)
            );
        }

        if result != 0 {
            println!("__vmx_vmwrite error");
        }
        
        VmxInstructionResult::from(result)
    }

    pub fn __vmx_vmread(field: u64,value: &mut u64) -> VmxInstructionResult {
        let mut result:u64;
        unsafe {
            asm!(
                "xor rax,rax",
                "vmread [rdx],rcx",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") field,
                in("rdx") value,
                out("rax") result,
                options(nostack, nomem)
            );
        }
        
        VmxInstructionResult::from(result)
    }

    pub fn vmcs_read(field: u64) -> u64 {
        let mut v:u64 = 0;
        let r = __vmx_vmread(field,&mut v);
        match r {
            VmxInstructionResult::VmxSuccess => {
                return v;
            },
            _ => {
                return 0;
            }
        }
    }

    pub fn __vmx_vmcall(vmcall_no: u64,arg1: u64,arg2: u64,arg3: u64) -> VmxInstructionResult {
        unsafe {
            asm!(
                "xor rax,rax",
                "vmcall",
                "setc al",
	            "setz cl",
	            "adc al,cl",
                in("rcx") vmcall_no,
                in("rdx") arg1,
                in("r8") arg2,
                in("r9") arg3,
                options(nostack, nomem)
            );
        }
        
        VmxInstructionResult::VmxFailValid
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

    pub fn __vmx_read_error() -> u64 {
        let mut error_code:u64 = 0;
        match __vmx_vmread(VM_INSTRUCTION_ERROR,&mut error_code) {
            VmxInstructionResult::VmxSuccess => {
                println!("Read ins error code success:{}",error_code);
                return error_code;
            },
            _ => {
                println!("error to read vmlaunch error code");
                return !0u64;
            }
        }
    }
}