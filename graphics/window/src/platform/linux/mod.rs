//! The Linux implementation of the window system

use crate::WindowEvents;
use shared_library::{SharedLibrary, try_load_function};

mod error;
mod shared_library;
mod wake_handle;

mod wayland;
mod x11;

mod builder;
mod deref;
mod get;
mod new;
mod process_messages;

pub(crate) use error::OsError;
pub(crate) use wake_handle::WindowWakeHandleInner;

pub use wayland::WaylandWindow;
pub use x11::X11Window;

/// A window displayed for the user
pub enum Window<Callbacks: WindowEvents = ()> {
    /// The protocol used is Wayland
    Wayland(WaylandWindow<Callbacks>),

    /// The protocol used is X11
    X11(X11Window),
}
