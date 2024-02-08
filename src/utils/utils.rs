use core::{arch::asm, ffi::c_void};

use alloc::vec::Vec;
use wdk_sys::{ntddk::{IoAllocateMdl, IoFreeMdl, KeGetCurrentProcessorNumberEx, MmBuildMdlForNonPagedPool, MmProtectMdlSystemAddress}, MDL_MAPPED_TO_SYSTEM_VA, NT_SUCCESS, UNICODE_STRING};

// utf16 str
pub fn string_to_utf16_slice(input: &str) -> Vec<u16> {
    let utf16_iter = input.encode_utf16();
    let utf16_vec: Vec<u16> = utf16_iter.collect();
    utf16_vec
}

// wdk unicode string
pub fn create_unicode_string(s: &[u16]) -> UNICODE_STRING {
    let len = s.len();

    let n = if len > 0 && s[len - 1] == 0 { len - 1 } else { len };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}

// eg. end_bit=5 result=0b11111
pub fn create_end_mask(end_bit: u64) -> u64 {
    // check range
    assert!(0 <= end_bit && end_bit < 64, "Invalid bit range");

    // calculate mask
    (!0u64) >> (64 - end_bit)
}

// eg 0b11_0000_0000 8 2 -> 0b11
pub fn get_bits_value(value:u64,shift:u64,len:u64) -> u64 {
    (value >> shift) & create_end_mask(len)
}

// eg 0b11_0000 3 1 1 -> 0b11_1000
// eg 0b1001_0000 6 2 0b11 -> 0b1111_0000
pub fn set_bits_value(value:u64,shift:u64,len:u64,new_value:u64) -> u64 {
    // create mask
    let mask = ((1u64 << len) - 1) << shift;
    // clean value 
    let cleared_value = value & !mask;
    // set new value
    let shifted_new_value = new_value << shift;
    // combine new value and clean value
    let result = cleared_value | shifted_new_value;
    result
}

pub fn get_current_processor_idx() -> u32 {
    return unsafe {
        KeGetCurrentProcessorNumberEx(core::ptr::null_mut())
    } as u32;
}

pub fn protect_non_paged_memory(ptr: *mut c_void,size: u64,protection: u32) -> Result<(),& 'static str>{
    let mdl = unsafe { IoAllocateMdl(ptr,size as _,false as _,false as _,core::ptr::null_mut()) };
    if mdl.is_null() {
        return Err("IoAllocateMdl error");
    }

    unsafe { MmBuildMdlForNonPagedPool(mdl) };
    unsafe { (*mdl).MdlFlags |= MDL_MAPPED_TO_SYSTEM_VA as i16; } 
    let status = unsafe { MmProtectMdlSystemAddress(mdl, protection) };
    unsafe{ IoFreeMdl(mdl) };
    if !NT_SUCCESS(status) {
        return Err("MmProtectMdlSystemAddress error");
    }

    Ok(())
}

pub fn __debugbreak() {
    unsafe{ 
        asm!{
            "int 3"
        };
    }
}