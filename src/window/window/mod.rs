use crate::{Id, window::WindowSubsystem};

mod builder;

#[cfg(target_os = "windows")]
mod windows;

pub use builder::WindowBuilder;

#[cfg(target_os = "windows")]
pub(in crate::window) use windows::{
    Win32Window, WindowClass, WindowInner, WindowProc, WindowStyle,
};

/// A reference to a window for which can be rendered into
#[derive(Clone)]
pub struct Window {
    /// The ID of the window being pointed at
    id: Id<WindowInner>,

    /// A reference to the context containing the window
    context: WindowSubsystem,
}
