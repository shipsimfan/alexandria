#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::Window;
#[cfg(target_os = "windows")]
pub use windows::Window;

#[cfg(target_os = "linux")]
pub(crate) use linux::OsError;
#[cfg(target_os = "windows")]
pub(crate) use windows::OsError;
