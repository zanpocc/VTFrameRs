use moon_log::info;

use super::{io_request::IoRequest, Device, DeviceOperations};

macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ((($DeviceType as u32) << 16)
            | (($Access as u32) << 14)
            | (($Function as u32) << 2)
            | ($Method as u32))
    };
}
const FILE_DEVICE_UNKNOWN: u32 = 0x00000022;
const METHOD_BUFFERED: u32 = 0;
// const METHOD_IN_DIRECT :u32  = 1;
// const METHOD_OUT_DIRECT:u32  = 2;
// const METHOD_NEITHER   :u32  = 3;
// const FILE_READ_ACCESS :u32 =  0x0001;     // file & pipe
// const FILE_WRITE_ACCESS:u32 =  0x0002;     // file & pipe
// const FILE_ANY_ACCESS  :u32 =  0;

const IOCTL_DEVICE_IO_CONTROL_TEST: u32 =
    CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x2000, METHOD_BUFFERED, 0);

pub struct IoControl {}

impl DeviceOperations for IoControl {
    fn create(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        info!("create dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn close(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        info!("close dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn cleanup(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        info!("cleanup dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn others(&self, _device: &Device, request: &mut IoRequest) -> Result<(), &'static str> {
        info!("IoCtl");

        let code = request.control_code();
        let buff = request.system_buffer();
        let _input_data_length = request.input_buffer_length();
        let _output_data_length = request.output_buffer_length();

        let mut ret = 0;

        if code == IOCTL_DEVICE_IO_CONTROL_TEST {
            info!("Test DeviceControl");
            let p: *mut DeviceIoTestOut = buff as _;
            unsafe {
                info!("{},{}", (*p).length, (*p).maximum_length);
            }

            let out_p: *mut DeviceIoTestOut = buff as _;
            (unsafe { &mut *out_p }).length = 1;
            (unsafe { &mut *out_p }).maximum_length = 2;

            ret = core::mem::size_of::<DeviceIoTestOut>();
        }

        request.complete(Ok(ret));
        Ok(())
    }
}

#[repr(C)]
struct DeviceIoTestOut {
    length: u16,         // version
    maximum_length: u16, // vmx abort reason. vmx abort:vmexit fault
}
