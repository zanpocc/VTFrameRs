pub mod data_struct;

use moon_struct::inner::PKPROCESSOR_STATE;
// kernel inner function
use wdk_sys::{LIST_ENTRY, PCONTEXT, PIO_STACK_LOCATION, PIRP, _EXCEPTION_RECORD};

pub fn io_get_current_irp_stack_location(irp: PIRP) -> PIO_STACK_LOCATION {
    unsafe {
        (*irp)
            .Tail
            .Overlay
            .__bindgen_anon_2
            .__bindgen_anon_1
            .CurrentStackLocation
    }
}

pub fn initialize_list_head(list_head: &mut LIST_ENTRY) {
    list_head.Flink = list_head;
    list_head.Blink = list_head;
}

extern "C" {
    pub fn KeSaveStateForHibernate(state: PKPROCESSOR_STATE);
    pub fn RtlRestoreContext(ContextRecord: PCONTEXT, ExceptionRecord: *mut _EXCEPTION_RECORD);
}
