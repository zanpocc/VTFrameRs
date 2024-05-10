use core::ffi::c_void;

use wdk_sys::NTSTATUS;

extern "C" {
    // NTSTATUS TestDistorm();
    // ULONG DistormAsmLength(PVOID TargetAddress, ULONG size);
    pub fn TestDistorm() -> NTSTATUS;
    pub fn DistormAsmLength(TargetAddress: *mut c_void, size: u32) -> u32;
}