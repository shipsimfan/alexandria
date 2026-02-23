use crate::FunctionSymbol;
use wayland::{
    WlDisplayCancelRead, WlDisplayConnect, WlDisplayDisconnect, WlDisplayDispatchPending,
    WlDisplayFlush, WlDisplayGetError, WlDisplayGetFd, WlDisplayPrepareRead, WlDisplayReadEvents,
    WlDisplayRoundtrip, WlProxyAddListener, WlProxyGetVersion, WlProxyMarshalFlags,
};

mod load;

/// All functions that must be loaded for `Wayland`
pub(in crate::window) struct WaylandFunctions {
    /// The function to connect to Wayland
    pub display_connect: FunctionSymbol<WlDisplayConnect>,

    /// The function to get the last error on the connection
    pub display_get_error: FunctionSymbol<WlDisplayGetError>,

    /// The function to get the underlying file descriptor of a display
    pub display_get_fd: FunctionSymbol<WlDisplayGetFd>,

    /// The function to block until all outstanding requests are processed
    pub display_roundtrip: FunctionSymbol<WlDisplayRoundtrip>,

    /// The function to prepare the display for reading events
    pub display_prepare_read: FunctionSymbol<WlDisplayPrepareRead>,

    /// The function to flush all waiting requests on the display
    pub display_flush: FunctionSymbol<WlDisplayFlush>,

    /// The function to actively read events from socket
    pub display_read_events: FunctionSymbol<WlDisplayReadEvents>,

    /// The function to cancel a prepared read request
    pub display_cancel_read: FunctionSymbol<WlDisplayCancelRead>,

    /// The function to dispatch all pending events
    pub display_dispatch_pending: FunctionSymbol<WlDisplayDispatchPending>,

    /// The function to destroy the `wl_display` and end the connection
    pub display_disconnect: FunctionSymbol<WlDisplayDisconnect>,

    /// The function to marshal requests to Wayland
    pub proxy_marshal_flags: FunctionSymbol<WlProxyMarshalFlags>,

    /// The function to get the version of a Wayland proxy
    pub proxy_get_version: FunctionSymbol<WlProxyGetVersion>,

    /// The function to add a listener to a Wayland proxy
    pub proxy_add_listener: FunctionSymbol<WlProxyAddListener>,
}
