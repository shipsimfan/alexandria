use wayland::{WlDisplayConnect, WlDisplayDisconnect};

mod load;

/// All functions that must be loaded for `Wayland`
pub(in crate::platform::linux) struct WaylandFunctions {
    /// The function to connect to Wayland
    pub display_connect: WlDisplayConnect,

    /// The function to destroy the `wl_display` and end the connection
    pub display_disconnect: WlDisplayDisconnect,
}
