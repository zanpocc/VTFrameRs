use alloc::vec::Vec;
use wdk_sys::UNICODE_STRING;

pub fn string_to_utf16_slice(input: &str) -> Vec<u16> {
    let utf16_iter = input.encode_utf16();
    let utf16_vec: Vec<u16> = utf16_iter.collect();
    utf16_vec
}

pub fn create_unicode_string(s: &[u16]) -> UNICODE_STRING {
    let len = s.len();

    let n = if len > 0 && s[len - 1] == 0 { len - 1 } else { len };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}


pub fn create_mask(start_bit: usize, end_bit: usize) -> u64 {
    // check range
    assert!(start_bit <= end_bit && end_bit < 64, "Invalid bit range");

    // calu mask
    let mut mask = 0;
    for i in start_bit..=end_bit {
        mask |= 1 << i;
    }

    mask
}

// start_bit = 0
pub fn create_end_mask(end_bit: usize) -> u64 {
    // check range
    assert!(0 <= end_bit && end_bit < 63, "Invalid bit range");

    // calu mask
    let mut mask = 0;
    mask = (1 << (end_bit + 1))-1;
    mask
}