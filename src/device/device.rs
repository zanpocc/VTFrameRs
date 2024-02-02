use alloc::boxed::Box;
use wdk::println;
use wdk_sys::{ntddk::IoDeleteDevice, DEVICE_OBJECT};

pub struct Device{
    pub raw: *mut DEVICE_OBJECT,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Drop for Device {
    fn drop(&mut self) {
        println!("Start drop device");

        if self.raw.is_null() {
            return;
        }

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

    pub unsafe fn into_raw(mut self) -> *mut DEVICE_OBJECT {
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
    
    // pub(crate) fn vtable(&self) -> &DeviceOperationsImpl {
    //     unsafe {
    //         &*(self.extension().vtable as *const _)
    //     }
    // }
    
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


// device data operation
pub trait DeviceOperations {
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

        println!("release device point success");
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
    release: Option<extern "C" fn (*mut DEVICE_OBJECT)>,
}

// stcuct of vtable
pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);
impl <T: DeviceOperations> DeviceOperationsVtable<T> {
    pub(crate) const VTABLE: DeviceOperationsImpl = DeviceOperationsImpl {
        release: Some(release_callback::<T>),
    };
}