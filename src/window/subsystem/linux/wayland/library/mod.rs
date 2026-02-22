use crate::SharedObject;

mod functions;

mod try_open;

pub(in crate::window::subsystem::linux::wayland) use functions::WaylandFunctions;

/// Reference to `libwayland-client.so`
pub(in crate::window::subsystem::linux::wayland) struct WaylandLibrary {
    /// The handle to the library containing the functions
    #[allow(unused)]
    library: SharedObject,

    /// The functions used by Wayland
    pub f: WaylandFunctions,
}
