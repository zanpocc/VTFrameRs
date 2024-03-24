use moon_log::info;

use super::{device::{Device, DeviceOperations}, io_request::IoRequest};

pub struct IoControl{}

impl DeviceOperations for IoControl{
    fn create(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        info!("create dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn close(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        info!("close dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn cleanup(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        info!("cleanup dispatch");
        request.complete(Ok(0));
        Ok(())
    }
}