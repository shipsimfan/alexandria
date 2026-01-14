//! The Linux implementation of the window system

use shared_library::{SharedLibrary, try_load_function};
use wayland::WaylandWindow;
use x11::X11Window;

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

use crate::WindowEvents;

/// A window displayed for the user
pub struct Window<Callbacks: WindowEvents = ()> {
    /// The underlying kind of window this is
    kind: WindowKind<Callbacks>,
}

/// The backend protocol for a given window
enum WindowKind<Callbacks: WindowEvents> {
    /// The protocol used is Wayland
    Wayland(WaylandWindow<Callbacks>),

    /// The protocol used is X11
    X11(X11Window),
}
