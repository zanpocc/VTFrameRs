extern crate alloc;

use core::{ffi::CStr, slice, u8};

use alloc::{string::String, vec::Vec};
use wdk::println;
use wdk_sys::UNICODE_STRING;

pub fn string_to_u16_slice(input: &str) -> Vec<u16> {
    let utf16_iter = input.encode_utf16();
    let utf16_vec: Vec<u16> = utf16_iter.collect();
    utf16_vec
}

pub fn u16_slice_to_unicode_string(s: &[u16]) -> UNICODE_STRING {
    let len = s.len();

    let n = if len > 0 && s[len - 1] == 0 {
        len - 1
    } else {
        len
    };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}

pub fn u16_slice_to_string(s: &[u16]) -> String {
    match String::from_utf16(s) {
        Ok(s) => return s,
        Err(_) => {
            println!("from utf16 error");
        }
    }

    return String::new();
}

pub fn str_to_unicode_string(s: &str) -> UNICODE_STRING {
    let s = string_to_u16_slice(s);

    let len = s.len();

    let n = if len > 0 && s[len - 1] == 0 {
        len - 1
    } else {
        len
    };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}

pub fn unicode_string_to_string(s: &UNICODE_STRING) -> String {
    let buffer_slice = unsafe { slice::from_raw_parts(s.Buffer, s.Length as usize / 2) };
    return u16_slice_to_string(buffer_slice);
}

pub fn cstr_to_rust_str(cstr_ptr: *mut u8) -> String {
    unsafe {
        let c_str = CStr::from_ptr(cstr_ptr as _);
        c_str.to_string_lossy().into_owned()
    }
}
