#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub(in crate::window) use windows::{Win32Window, WindowClass, WindowProc};
