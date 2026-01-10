use wayland::{
    WlDisplayCancelRead, WlDisplayConnect, WlDisplayDisconnect, WlDisplayDispatchPending,
    WlDisplayFlush, WlDisplayGetError, WlDisplayGetFd, WlDisplayPrepareRead, WlDisplayReadEvents,
    WlDisplayRoundtrip, WlProxyAddListener, WlProxyGetVersion, WlProxyMarshalFlags,
};

mod load;

/// All functions that must be loaded for `Wayland`
pub(in crate::platform::linux) struct WaylandFunctions {
    /// The function to connect to Wayland
    pub display_connect: WlDisplayConnect,

    /// The function to get the last error on the connection
    pub display_get_error: WlDisplayGetError,

    /// The function to get the underlying file descriptor of a display
    pub display_get_fd: WlDisplayGetFd,

    /// The function to block until all outstanding requests are processed
    pub display_roundtrip: WlDisplayRoundtrip,

    /// The function to prepare the display for reading events
    pub display_prepare_read: WlDisplayPrepareRead,

    /// The function to flush all waiting requests on the display
    pub display_flush: WlDisplayFlush,

    /// The function to actively read events from socket
    pub display_read_events: WlDisplayReadEvents,

    /// The function to cancel a prepared read request
    pub display_cancel_read: WlDisplayCancelRead,

    /// The function to dispatch all pending events
    pub display_dispatch_pending: WlDisplayDispatchPending,

    /// The function to destroy the `wl_display` and end the connection
    pub display_disconnect: WlDisplayDisconnect,

    /// The function to marshal requests to Wayland
    pub proxy_marshal_flags: WlProxyMarshalFlags,

    /// The function to get the version of a Wayland proxy
    pub proxy_get_version: WlProxyGetVersion,

    /// The function to add a listener to a Wayland proxy
    pub proxy_add_listener: WlProxyAddListener,
}
