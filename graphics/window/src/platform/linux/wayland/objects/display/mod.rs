use crate::platform::linux::wayland::WaylandLibrary;
use wayland::wl_display;

mod drop;
mod try_connect;

/// The main connection to Wayland
pub(in crate::platform::linux) struct WlDisplay {
    /// The handle to the display
    handle: *mut wl_display,

    /// The reference to the client library
    pub(in crate::platform::linux::wayland) library: WaylandLibrary,
}
