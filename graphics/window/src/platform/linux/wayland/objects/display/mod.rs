use crate::platform::linux::wayland::WaylandLibrary;
use std::cell::RefCell;
use wayland::wl_display;

mod dispatch;
mod drop;
mod get_registry;
mod roundtrip;
mod try_connect;

/// The main connection to Wayland
pub(in crate::platform::linux) struct WlDisplay {
    /// The handle to the display
    handle: RefCell<*mut wl_display>,

    /// The reference to the client library
    pub(in crate::platform::linux::wayland) library: WaylandLibrary,
}
