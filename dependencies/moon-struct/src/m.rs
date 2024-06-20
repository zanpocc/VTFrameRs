#[macro_export]
macro_rules! RT_BIT_32 {
    ($bit:expr) => {
        1u32 << ($bit)
    };
}

#[macro_export]
macro_rules! RT_BIT_64 {
    ($bit:expr) => {
        1u64 << ($bit)
    };
}

#[macro_export]
macro_rules! RT_BIT {
    ($bit:expr) => {
        1u << ($bit)
    };
}
