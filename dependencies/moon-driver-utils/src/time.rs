extern crate alloc;

use alloc::string::String;
use moon_struct::constant::ke_query_system_time;
use wdk_sys::{ntddk::RtlTimeToTimeFields, LARGE_INTEGER, TIME_FIELDS};

pub fn get_current_time() -> String {

    let mut system_time:LARGE_INTEGER = LARGE_INTEGER::default();
    let time_fields:TIME_FIELDS = TIME_FIELDS::default();

    unsafe {
        system_time.QuadPart = ke_query_system_time() as _;
        RtlTimeToTimeFields(&system_time as *const _ as _, &time_fields as *const _ as _);
    }

    alloc::format!("{}{}{}{}{}{}",
        time_fields.Year,
        time_fields.Month,
        time_fields.Day,
        time_fields.Hour,
        time_fields.Minute,
        time_fields.Second
    )
}