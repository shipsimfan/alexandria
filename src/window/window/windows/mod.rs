mod class;
mod win32;

pub(in crate::window) use class::{WindowClass, WindowProc};
pub(in crate::window) use win32::Win32Window;
