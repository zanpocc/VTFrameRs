

// eg. end_bit=5 result=0b11111
pub fn create_end_mask(end_bit: u64) -> u64 {
    // calculate mask
    (!0u64) >> (64 - end_bit)
}

pub fn create_end_mask32(end_bit: u32) -> u32 {
    // calculate mask
    (!0u32) >> (32 - end_bit)
}

// eg 0b11_0000_0000 8 2 -> 0b11
pub fn get_bits_value(value:u64,shift:u64,len:u64) -> u64 {
    (value >> shift) & create_end_mask(len)
}

// eg 0b11_0000 3 1 1 -> 0b11_1000
// eg 0b1001_0000 6 2 0b11 -> 0b1111_0000
pub fn set_bits_value(value:u64,shift:u64,len:u64,new_value:u64) -> u64 {
    let new_value = new_value & create_end_mask(len);

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

pub fn set_bits_value32(value:u32,shift:u32,len:u32,new_value:u32) -> u32 {
    let new_value = new_value & create_end_mask32(len);

    // create mask
    let mask = ((1u32 << len) - 1) << shift;
    // clean value 
    let cleared_value = value & !mask;
    // set new value
    let shifted_new_value = new_value << shift;
    // combine new value and clean value
    let result = cleared_value | shifted_new_value;
    result
}