#[cfg(not(windows))]
mod non_windows;
#[cfg(windows)]
mod windows;

use crate::Result;
use std::ffi::c_void;

#[cfg(not(windows))]
pub use self::non_windows::EventObject;
#[cfg(windows)]
pub use self::windows::EventObject;

pub trait EventObjectTrait: Sized {
    fn new() -> Result<Self>;

    fn wait(&self) -> Result<()>;

    fn as_ptr(&self) -> *mut c_void;
}
