use wdk_sys::{ntddk::IofCompleteRequest, IO_NO_INCREMENT, IO_STACK_LOCATION, IRP, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::inner::io_get_current_irp_stack_location;

pub struct IoRequest {
    raw: *mut IRP,
}

impl IoRequest {
    pub fn complete(&self, value: Result<u32, & 'static str>) {
        let irp = self.as_raw_mut();

        match value {
            Ok(value) => {
                irp.IoStatus.Information = value as _;
                irp.IoStatus.__bindgen_anon_1.Status = STATUS_SUCCESS;
            }
            Err(_) => {
                irp.IoStatus.Information = 0;
                irp.IoStatus.__bindgen_anon_1.Status = STATUS_UNSUCCESSFUL;
            }
        }

        unsafe {
            IofCompleteRequest(irp, IO_NO_INCREMENT as _);
        }
    }

    pub unsafe fn from_raw(irp: *mut IRP) -> Self {
        Self { raw: irp }
    }

    pub fn as_raw(&self) -> &IRP {
        unsafe { &*self.raw }
    }

    pub fn as_raw_mut(&self) -> &mut IRP {
        unsafe { &mut *self.raw }
    }

    pub fn stack_location(&self) -> &IO_STACK_LOCATION {
        unsafe { &*io_get_current_irp_stack_location(self.raw) }
    }

    pub fn major(&self) -> u8 {
        self.stack_location().MajorFunction
    }
}