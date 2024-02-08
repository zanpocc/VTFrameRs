pub mod data_struct;

// kernel inner function
use wdk_sys::{PCONTEXT, PIO_STACK_LOCATION, PIRP, _EXCEPTION_RECORD};

use self::data_struct::PKPROCESSOR_STATE;

pub fn io_get_current_irp_stack_location(irp: PIRP) -> PIO_STACK_LOCATION{
    unsafe{
        (*irp).Tail.Overlay.__bindgen_anon_2.__bindgen_anon_1.CurrentStackLocation
    }
}

extern "C" {
    pub fn KeSaveStateForHibernate(state: PKPROCESSOR_STATE);
    pub fn RtlRestoreContext(ContextRecord: PCONTEXT,ExceptionRecord: *mut _EXCEPTION_RECORD);
}