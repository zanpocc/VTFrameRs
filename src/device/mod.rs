pub mod io_request;
pub mod ioctl;
pub mod symbolic_link;

use alloc::boxed::Box;
use moon_log::info;
use wdk_sys::{
    ntddk::IoDeleteDevice, DEVICE_OBJECT, IRP, IRP_MJ_CLEANUP, IRP_MJ_CLOSE, IRP_MJ_CREATE,
    NTSTATUS, STATUS_SUCCESS, STATUS_UNSUCCESSFUL,
};

use crate::inner::io_get_current_irp_stack_location;

use io_request::IoRequest;

pub struct Device {
    pub raw: *mut DEVICE_OBJECT,
}

impl Drop for Device {
    fn drop(&mut self) {
        // must use into_raw() set raw point to null after Device::from_raw()
        // otherwise will clean deviceextension memory
        if self.raw.is_null() {
            return;
        }

        info!("Start drop device");

        unsafe {
            // Release Data
            let extension = (*self.raw).DeviceExtension as *mut DeviceExtension;
            let vtable = (*extension).vtable;

            // free memory in vtable function
            if let Some(release) = (*vtable).release {
                release(self.raw);
            }

            IoDeleteDevice(self.raw);
        }
    }
}

impl Device {
    pub fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self { raw }
    }

    pub fn as_raw(&self) -> *const DEVICE_OBJECT {
        self.raw as *const _
    }

    pub fn as_raw_mut(&self) -> *mut DEVICE_OBJECT {
        self.raw
    }

    pub fn into_raw(mut self) -> *mut DEVICE_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }

    pub(crate) fn extension(&self) -> &DeviceExtension {
        unsafe { &*((*self.raw).DeviceExtension as *const DeviceExtension) }
    }

    pub(crate) fn extension_mut(&mut self) -> &mut DeviceExtension {
        unsafe { &mut *((*self.raw).DeviceExtension as *mut DeviceExtension) }
    }

    pub(crate) fn vtable(&self) -> &DeviceOperationsImpl {
        unsafe { &*(self.extension().vtable as *const _) }
    }

    pub fn data<T: DeviceOperations>(&self) -> &T {
        unsafe { &*(self.extension().data as *const T) }
    }

    pub fn data_mut<T: DeviceOperations>(&mut self) -> &mut T {
        unsafe { &mut *(self.extension().data as *mut T) }
    }
}

/// # Safety
///
/// derefrence irp
pub unsafe extern "C" fn dispatch_device(device: *mut DEVICE_OBJECT, irp: *mut IRP) -> NTSTATUS {
    let stack_location = unsafe { &*io_get_current_irp_stack_location(irp) };
    let device = Device::from_raw(device);
    let vtable = device.vtable();

    match vtable.dispatch {
        // device must into_raw(),too dangerous
        Some(dispatch) => dispatch(device.into_raw(), irp, stack_location.MajorFunction),
        _ => {
            device.into_raw();
            STATUS_SUCCESS
        }
    }
}

// default dispatch operation
pub trait DeviceOperations {
    fn create(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        request.complete(Ok(0));
        Ok(())
    }

    fn close(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        request.complete(Ok(0));
        Ok(())
    }

    fn cleanup(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        request.complete(Ok(0));

        Ok(())
    }

    fn others(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        request.complete(Ok(0));

        Ok(())
    }
}

// dispatch irp
extern "C" fn dispatch_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
    major: u8,
) -> NTSTATUS {
    let device = Device::from_raw(device);
    let data: &T = device.data();
    let mut request = IoRequest::from_raw(irp);

    let device_ref = &device;

    let status = match major as _ {
        IRP_MJ_CREATE => data.create(device_ref, &mut request),
        IRP_MJ_CLOSE => data.close(device_ref, &mut request),
        IRP_MJ_CLEANUP => data.cleanup(device_ref, &mut request),
        _ => data.others(device_ref, &mut request),
    };

    // must into,too dangerous
    device.into_raw();

    match status {
        Ok(()) => STATUS_SUCCESS,
        Err(e) => {
            request.complete(Err(e));
            STATUS_UNSUCCESSFUL
        }
    }
}

// free memory when device drop
extern "C" fn release_callback<T: DeviceOperations>(device: *mut DEVICE_OBJECT) {
    unsafe {
        let extension = (*device).DeviceExtension as *mut DeviceExtension;

        // auto free data memory in device object
        let ptr = core::mem::replace(&mut (*extension).data, core::ptr::null_mut());
        let _ = Box::from_raw(ptr as *mut T);

        info!("release device point success");
    }
}

// data releation with device
#[repr(C)]
pub struct DeviceExtension {
    pub(crate) vtable: *const DeviceOperationsImpl, // virtual table function point
    pub(crate) data: *mut cty::c_void,              // user data
}

#[repr(C)]
pub struct DeviceOperationsImpl {
    dispatch: Option<extern "C" fn(*mut DEVICE_OBJECT, *mut IRP, u8) -> NTSTATUS>,
    release: Option<extern "C" fn(*mut DEVICE_OBJECT)>,
}

// stcuct of vtable
pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);
impl<T: DeviceOperations> DeviceOperationsVtable<T> {
    pub(crate) const VTABLE: DeviceOperationsImpl = DeviceOperationsImpl {
        dispatch: Some(dispatch_callback::<T>),
        release: Some(release_callback::<T>),
    };
}
