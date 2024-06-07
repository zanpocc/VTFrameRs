
pub const KI_USER_SHARED_DATA: u64 = 0xFFFFF78000000000;
pub const SHARED_INTERRUPT_TIME: u64 = KI_USER_SHARED_DATA + 0x8;
pub const SHARED_SYSTEM_TIME: u64 = KI_USER_SHARED_DATA + 0x14;
pub const SHARED_TICK_COUNT: u64 = KI_USER_SHARED_DATA + 0x320;

pub fn ke_query_interrupt_time() -> u64 {
    unsafe{
        return *(SHARED_INTERRUPT_TIME as *mut u64);
    }
}

pub fn ke_query_system_time() -> u64 {
    unsafe{
        return *(SHARED_SYSTEM_TIME as *mut u64);
    }
}

pub fn ke_query_tick_count() -> u64 {
    unsafe{
        return *(SHARED_TICK_COUNT as *mut u64);
    }
}