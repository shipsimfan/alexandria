use crate::{Id, window::WindowSubsystem};

mod builder;
mod iter;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod destroy;
mod get;
mod inner;
mod new;

pub use builder::WindowBuilder;
pub use iter::WindowIter;

#[cfg(target_os = "linux")]
pub(in crate::window) use linux::WindowInner;
#[cfg(target_os = "windows")]
pub(in crate::window) use windows::{
    StandardWndProc, Win32Window, WindowClass, WindowInner, WindowProc, WindowStyle,
};

/// A reference to a window for which can be rendered into
#[derive(Clone)]
pub struct Window<UserEvent: 'static + Send> {
    /// The ID of the window being pointed at
    id: Id<Window<UserEvent>>,

    /// A reference to the context containing the window
    context: WindowSubsystem<UserEvent>,
}
