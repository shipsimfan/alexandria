use crate::platform::linux::{
    SharedLibrary, try_load_function, wayland::library::WaylandFunctions,
};
use wayland::{
    WL_DISPLAY_CONNECT, WL_DISPLAY_DISCONNECT, WL_DISPLAY_DISPATCH, WL_DISPLAY_GET_ERROR,
    WL_DISPLAY_ROUNDTRIP, WL_PROXY_ADD_LISTENER, WL_PROXY_GET_VERSION, WL_PROXY_MARSHAL_FLAGS,
};

impl WaylandFunctions {
    /// Load all the functions needed for Wayland
    pub fn load(library: &mut SharedLibrary) -> Option<WaylandFunctions> {
        Some(WaylandFunctions {
            display_connect: try_load_function!(library, WL_DISPLAY_CONNECT)?,
            display_get_error: try_load_function!(library, WL_DISPLAY_GET_ERROR)?,
            display_roundtrip: try_load_function!(library, WL_DISPLAY_ROUNDTRIP)?,
            display_dispatch: try_load_function!(library, WL_DISPLAY_DISPATCH)?,
            display_disconnect: try_load_function!(library, WL_DISPLAY_DISCONNECT)?,

            proxy_marshal_flags: try_load_function!(library, WL_PROXY_MARSHAL_FLAGS)?,
            proxy_get_version: try_load_function!(library, WL_PROXY_GET_VERSION)?,
            proxy_add_listener: try_load_function!(library, WL_PROXY_ADD_LISTENER)?,
        })
    }
}
