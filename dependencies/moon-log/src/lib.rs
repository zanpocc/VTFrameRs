#![no_std]

pub mod buffer;
pub mod event;

extern crate alloc;

use alloc::ffi::CString;
use wdk_sys::ntddk::DbgPrint;


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let formatted_string = CString::new(alloc::format!("{args}"))
        .expect("CString should be able to be created from a String.");

    unsafe {
        DbgPrint(formatted_string.as_ptr());
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
      ($crate::_print(format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        ($crate::print!("\n"));
    };
    ($($arg:tt)*) => {
        ($crate::print!("{}\n", format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! log_message {
    ($level:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::print!("{:<5} {:<30}:{:<5} {}\n", $level, file!(), line!(), format_args!($($arg)*));
        }
        // #[cfg(not(debug_assertions))]
        // {
        //     $crate::print!("{:<5} {}\n", $level, format_args!($($arg)*));
        // }
    };
}

#[macro_export]
macro_rules! info {
    () => {
        println!("");
    };
    ($($arg:tt)*) => {
        $crate::log_message!("Info", $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        {
            println!("");
        }
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::log_message!("Debug", $($arg)*);    
        }
    };
}

#[macro_export]
macro_rules! warn {
    () => {
        println!("");
    };
    ($($arg:tt)*) => {
        $crate::log_message!("Warn", $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    () => {
        println!("");
    };
    ($($arg:tt)*) => {
        $crate::log_message!("Error", $($arg)*);
    };
}


