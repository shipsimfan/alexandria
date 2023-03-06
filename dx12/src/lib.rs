mod adapter;
mod instance;

pub use adapter::*;
pub use instance::*;

pub type Error = win32::Win32Error;
