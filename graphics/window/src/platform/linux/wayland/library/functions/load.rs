use crate::platform::linux::{
    SharedLibrary, try_load_function, wayland::library::WaylandFunctions,
};
use wayland::{WL_DISPLAY_CONNECT, WL_DISPLAY_DISCONNECT};

impl WaylandFunctions {
    /// Load all the functions needed for Wayland
    pub fn load(library: &mut SharedLibrary) -> Option<WaylandFunctions> {
        Some(WaylandFunctions {
            display_connect: try_load_function!(library, WL_DISPLAY_CONNECT)?,
            display_disconnect: try_load_function!(library, WL_DISPLAY_DISCONNECT)?,
        })
    }
}
