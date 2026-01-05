use wayland::{
    WlDisplayConnect, WlDisplayDisconnect, WlDisplayDispatch, WlDisplayGetError,
    WlProxyAddListener, WlProxyGetVersion, WlProxyMarshalFlags,
};

mod load;

/// All functions that must be loaded for `Wayland`
pub(in crate::platform::linux) struct WaylandFunctions {
    /// The function to connect to Wayland
    pub display_connect: WlDisplayConnect,

    /// The function to get the last error on the connection
    pub display_get_error: WlDisplayGetError,

    /// The function to dispatch all waiting events
    pub display_dispatch: WlDisplayDispatch,

    /// The function to destroy the `wl_display` and end the connection
    pub display_disconnect: WlDisplayDisconnect,

    /// The function to marshal requests to Wayland
    pub proxy_marshal_flags: WlProxyMarshalFlags,

    /// The function to get the version of a Wayland proxy
    pub proxy_get_version: WlProxyGetVersion,

    /// The function to add a listener to a Wayland proxy
    pub proxy_add_listener: WlProxyAddListener,
}
