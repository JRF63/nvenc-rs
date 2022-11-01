use super::EventObjectTrait;
use crate::{NvEncError, Result};
use std::ffi::c_void;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
    System::Threading::{CreateEventA, WaitForSingleObject},
};

#[repr(transparent)]
pub struct EventObject(HANDLE);

impl Drop for EventObject {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

impl EventObjectTrait for EventObject {
    fn new() -> Result<Self> {
        match unsafe { CreateEventA(None, false, false, None) } {
            Ok(event) => Ok(EventObject(event)),
            Err(_) => Err(NvEncError::EventObjectCreationFailed),
        }
    }

    fn wait(&self, timeout_millis: u32) -> Result<()> {
        match unsafe { WaitForSingleObject(self.0, timeout_millis) } {
            WAIT_OBJECT_0 => Ok(()),
            WAIT_TIMEOUT => Err(NvEncError::EventObjectWaitTimeout),
            _ => Err(NvEncError::EventObjectWaitError),
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.0 .0 as *mut c_void
    }
}