use core::arch::asm;

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
    // 创建掩码，将要修改的位设置为1
    let mask = ((1u64 << len) - 1) << shift;

    // 清除要修改的位置的值
    let cleared_value = value & !mask;

    // 设置新值
    let shifted_new_value = new_value << shift;

    // 结合清除的值和新值
    let result = cleared_value | shifted_new_value;

    result
}

pub fn __debugbreak() {
    unsafe{ 
        asm!{
            "int 3"
        };
    }
}