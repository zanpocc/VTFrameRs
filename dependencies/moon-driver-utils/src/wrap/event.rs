use core::ops::{Deref, DerefMut};

use wdk_sys::{ntddk::{IoCreateNotificationEvent, KeClearEvent}, PKEVENT};

use super::handle::Handle;

pub struct Event {
    raw: PKEVENT,
    #[allow(unused)]
    h: Handle,
}

impl Event {

    pub fn new() -> Result<Self, &'static str> {
        let mut event_handle = Handle::default();
        unsafe {
            let event =  IoCreateNotificationEvent(core::ptr::null_mut(), event_handle.as_ptr());
            if event.is_null(){
                return Err("IoCreateNotificationEvent Error");
            }

            KeClearEvent(event);

            let r = Self{
                raw: event,
                h: event_handle,
            };

            return Ok(r);
        }
    }

    pub fn from_raw(raw: PKEVENT,h: Handle) -> Self {
        unsafe {
            KeClearEvent(raw);
        }

        Self { 
            raw,
            h,
        }
    }

    pub fn as_mut_raw(&mut self) -> PKEVENT {
        *(&mut self.raw)
    }
}

impl Deref for Event {
    type Target = PKEVENT;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        if !self.raw.is_null(){
        }
    }
}
