
use alloc::boxed::Box;
use moon_log::info;
use wdk_sys::{ntddk::IoDeleteDevice, DEVICE_OBJECT, IRP, IRP_MJ_CLEANUP, IRP_MJ_CLOSE, IRP_MJ_CREATE, NTSTATUS, STATUS_SUCCESS, STATUS_UNSUCCESSFUL};

use crate::inner::io_get_current_irp_stack_location;

use super::io_request::IoRequest;

pub struct Device{
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
    pub unsafe fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self{
            raw,
        }
    }

    pub unsafe fn as_raw(&self) -> *const DEVICE_OBJECT {
        self.raw as *const _
    }

    pub unsafe fn as_raw_mut(&self) -> *mut DEVICE_OBJECT {
        self.raw
    }

    pub fn into_raw(mut self) -> *mut DEVICE_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }

    pub(crate) fn extension(&self) -> &DeviceExtension {
        unsafe {
            &*((*self.raw).DeviceExtension as *const DeviceExtension)
        }
    }
    
    pub(crate) fn extension_mut(&self) -> &mut DeviceExtension {
        unsafe {
            &mut *((*self.raw).DeviceExtension as *mut DeviceExtension)
        }
    }
    
    pub(crate) fn vtable(&self) -> &DeviceOperationsImpl {
        unsafe {
            &*(self.extension().vtable as *const _)
        }
    }
    
    pub fn data<T: DeviceOperations>(&self) -> &T {
        unsafe {
            &*(self.extension().data as *const T)
        }
    }
    
    pub fn data_mut<T: DeviceOperations>(&self) -> &mut T {
        unsafe {
            &mut *(self.extension().data as *mut T)
        }
    }
}


// dispatch entrypoint
pub extern "C" fn dispatch_device(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
) -> NTSTATUS {
    let stack_location = unsafe { &*io_get_current_irp_stack_location(irp) };
    let device = unsafe { Device::from_raw(device) };
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
    fn create(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        request.complete(Ok(0));
        Ok(())
    }

    fn close(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        request.complete(Ok(0));
        Ok(())
    }

    fn cleanup(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        request.complete(Ok(0));

        Ok(())
    }

    fn others(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
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
    let device = unsafe { Device::from_raw(device) };
    let data: &mut T = device.data_mut();
    let request = unsafe { IoRequest::from_raw(irp) };

    let status = match major as _ {
        IRP_MJ_CREATE => data.create(&device, &request),
        IRP_MJ_CLOSE => data.close(&device, &request),
        IRP_MJ_CLEANUP => data.cleanup(&device, &request),
        _ => data.others(&device, &request),
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
extern fn release_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) {
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
    pub(crate) data: *mut cty::c_void, // user data
}

#[repr(C)]
pub struct DeviceOperationsImpl {
    dispatch: Option<extern "C" fn (*mut DEVICE_OBJECT, *mut IRP, u8) -> NTSTATUS>,
    release: Option<extern "C" fn (*mut DEVICE_OBJECT)>,
}

// stcuct of vtable
pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);
impl <T: DeviceOperations> DeviceOperationsVtable<T> {
    pub(crate) const VTABLE: DeviceOperationsImpl = DeviceOperationsImpl {
        dispatch: Some(dispatch_callback::<T>),
        release: Some(release_callback::<T>),
    };
}