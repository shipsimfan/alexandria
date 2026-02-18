//! Subsystem for interacting with platform windowing systems

#[cfg(target_os = "windows")]
use window::{Win32Window, WindowClass, WindowProc, WindowStyle};

mod display;
mod subsystem;
mod window;

pub use display::{Display, DisplayIter, DisplayMode, DisplayOrientation};
pub use subsystem::WindowSubsystem;
