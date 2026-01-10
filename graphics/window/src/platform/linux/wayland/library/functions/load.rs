use crate::platform::linux::{
    SharedLibrary, try_load_function, wayland::library::WaylandFunctions,
};
use wayland::{
    WL_DISPLAY_CANCEL_READ, WL_DISPLAY_CONNECT, WL_DISPLAY_DISCONNECT, WL_DISPLAY_DISPATCH_PENDING,
    WL_DISPLAY_FLUSH, WL_DISPLAY_GET_ERROR, WL_DISPLAY_GET_FD, WL_DISPLAY_PREPARE_READ,
    WL_DISPLAY_READ_EVENTS, WL_DISPLAY_ROUNDTRIP, WL_PROXY_ADD_LISTENER, WL_PROXY_GET_VERSION,
    WL_PROXY_MARSHAL_FLAGS,
};

impl WaylandFunctions {
    /// Load all the functions needed for Wayland
    pub fn load(library: &mut SharedLibrary) -> Option<WaylandFunctions> {
        Some(WaylandFunctions {
            display_connect: try_load_function!(library, WL_DISPLAY_CONNECT)?,
            display_get_error: try_load_function!(library, WL_DISPLAY_GET_ERROR)?,
            display_get_fd: try_load_function!(library, WL_DISPLAY_GET_FD)?,
            display_roundtrip: try_load_function!(library, WL_DISPLAY_ROUNDTRIP)?,
            display_prepare_read: try_load_function!(library, WL_DISPLAY_PREPARE_READ)?,
            display_flush: try_load_function!(library, WL_DISPLAY_FLUSH)?,
            display_read_events: try_load_function!(library, WL_DISPLAY_READ_EVENTS)?,
            display_cancel_read: try_load_function!(library, WL_DISPLAY_CANCEL_READ)?,
            display_dispatch_pending: try_load_function!(library, WL_DISPLAY_DISPATCH_PENDING)?,
            display_disconnect: try_load_function!(library, WL_DISPLAY_DISCONNECT)?,

            proxy_marshal_flags: try_load_function!(library, WL_PROXY_MARSHAL_FLAGS)?,
            proxy_get_version: try_load_function!(library, WL_PROXY_GET_VERSION)?,
            proxy_add_listener: try_load_function!(library, WL_PROXY_ADD_LISTENER)?,
        })
    }
}
