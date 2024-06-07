#![no_std]

pub mod buffer;
pub mod event;

extern crate alloc;

#[macro_use]
extern crate lazy_static;

use alloc::ffi::CString;
use buffer::LOG;
use wdk_sys::ntddk::DbgPrint;


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let formatted_string = CString::new(alloc::format!("{args}"))
        .expect("CString should be able to be created from a String.");

    unsafe {
        DbgPrint(formatted_string.as_ptr());

        match *LOG {
            Some(ref log) => {
                let t = &mut *log.as_raw();
                t.write_log(args);
            }
            None => {}
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
      ($crate::_print(format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! myprintln {
    () => {
        ($crate::print!("\n"));
    };
    ($($arg:tt)*) => {
        ($crate::print!("{}\n", format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! info {
    () => {
        myprintln!("");
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::print!("Info {:<30}:{:<5} {}\n", file!(), line!(), format_args!($($arg)*));
        }
        // $crate::log_message!("Info", $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        {
            myprintln!("");
        }
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::print!("Debug {:<30}:{:<5} {}\n", file!(), line!(), format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! warn {
    () => {
        myprintln!("");
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::print!("Warn {:<30}:{:<5} {}\n", file!(), line!(), format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! error {
    () => {
        myprintln!("");
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::print!("Error {:<30}:{:<5} {}\n", file!(), line!(), format_args!($($arg)*));
        }
    };
}


