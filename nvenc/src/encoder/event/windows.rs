use super::EventObjectTrait;
use crate::{NvEncError, Result};
use std::ffi::c_void;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0},
    System::Threading::{CreateEventA, WaitForSingleObject},
    System::WindowsProgramming::INFINITE,
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

    fn wait(&self) -> Result<()> {
        match unsafe { WaitForSingleObject(self.0, INFINITE) } {
            WAIT_OBJECT_0 => Ok(()),
            _ => Err(NvEncError::EventObjectWaitError),
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.0 .0 as *mut c_void
    }
}
