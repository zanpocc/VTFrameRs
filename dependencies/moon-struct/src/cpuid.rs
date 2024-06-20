#[repr(C)]
#[derive(Default)]
pub struct CPUID {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}
