use crate::{Result, SharedObject, load_function, window::WaylandFunctions};
use wayland::{
    WL_DISPLAY_CANCEL_READ, WL_DISPLAY_CONNECT, WL_DISPLAY_DISCONNECT, WL_DISPLAY_DISPATCH_PENDING,
    WL_DISPLAY_FLUSH, WL_DISPLAY_GET_ERROR, WL_DISPLAY_GET_FD, WL_DISPLAY_PREPARE_READ,
    WL_DISPLAY_READ_EVENTS, WL_DISPLAY_ROUNDTRIP, WL_PROXY_ADD_LISTENER, WL_PROXY_GET_VERSION,
    WL_PROXY_MARSHAL_FLAGS,
};

impl WaylandFunctions {
    /// Load all the functions needed for Wayland
    pub fn load(library: &mut SharedObject) -> Result<WaylandFunctions> {
        Ok(WaylandFunctions {
            display_connect: load_function!(library, WL_DISPLAY_CONNECT)?,
            display_get_error: load_function!(library, WL_DISPLAY_GET_ERROR)?,
            display_get_fd: load_function!(library, WL_DISPLAY_GET_FD)?,
            display_roundtrip: load_function!(library, WL_DISPLAY_ROUNDTRIP)?,
            display_prepare_read: load_function!(library, WL_DISPLAY_PREPARE_READ)?,
            display_flush: load_function!(library, WL_DISPLAY_FLUSH)?,
            display_read_events: load_function!(library, WL_DISPLAY_READ_EVENTS)?,
            display_cancel_read: load_function!(library, WL_DISPLAY_CANCEL_READ)?,
            display_dispatch_pending: load_function!(library, WL_DISPLAY_DISPATCH_PENDING)?,
            display_disconnect: load_function!(library, WL_DISPLAY_DISCONNECT)?,

            proxy_marshal_flags: load_function!(library, WL_PROXY_MARSHAL_FLAGS)?,
            proxy_get_version: load_function!(library, WL_PROXY_GET_VERSION)?,
            proxy_add_listener: load_function!(library, WL_PROXY_ADD_LISTENER)?,
        })
    }
}
