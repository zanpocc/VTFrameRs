use wdk::println;

use super::{device::{Device, DeviceOperations}, io_request::IoRequest};

pub struct IoControl{}

impl DeviceOperations for IoControl{
    fn create(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        println!("create dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn close(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        println!("close dispatch");
        request.complete(Ok(0));
        Ok(())
    }

    fn cleanup(&mut self, _device: &Device, request: &IoRequest) -> Result<(), & 'static str> {
        println!("cleanup dispatch");
        request.complete(Ok(0));
        Ok(())
    }
}