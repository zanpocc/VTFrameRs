use core::ffi::c_void;

use wdk_sys::{
    ntddk::{
        IoAllocateMdl, IoFreeMdl, KeGetCurrentProcessorNumberEx, MmBuildMdlForNonPagedPool,
        MmGetPhysicalAddress, MmProtectMdlSystemAddress,
    },
    MDL_MAPPED_TO_SYSTEM_VA, NT_SUCCESS,
};

pub fn get_current_processor_idx() -> u32 {
    unsafe { KeGetCurrentProcessorNumberEx(core::ptr::null_mut()) as u32 }
}

/// # Safety
///
/// unsafe params ptr
pub unsafe fn protect_non_paged_memory(
    ptr: *mut c_void,
    size: u64,
    protection: u32,
) -> Result<(), &'static str> {
    let mdl = unsafe {
        IoAllocateMdl(
            ptr,
            size as _,
            false as _,
            false as _,
            core::ptr::null_mut(),
        )
    };
    if mdl.is_null() {
        return Err("IoAllocateMdl error");
    }

    unsafe { MmBuildMdlForNonPagedPool(mdl) };
    unsafe {
        (*mdl).MdlFlags |= MDL_MAPPED_TO_SYSTEM_VA as i16;
    }
    let status = unsafe { MmProtectMdlSystemAddress(mdl, protection) };
    unsafe { IoFreeMdl(mdl) };
    if !NT_SUCCESS(status) {
        return Err("MmProtectMdlSystemAddress error");
    }

    Ok(())
}

pub fn virtual_address_to_physical_address(virtual_address: *mut c_void) -> u64 {
    unsafe { MmGetPhysicalAddress(virtual_address as _).QuadPart as u64 }
}
