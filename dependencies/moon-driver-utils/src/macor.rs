#[macro_export]
macro_rules! page_align {
    ($va:expr) => {
        ($va as usize & !(4096 - 1)) as *mut u8
    };
}
