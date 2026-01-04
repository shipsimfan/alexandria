use crate::platform::linux::SharedLibrary;
use functions::WaylandFunctions;

mod functions;

mod try_open;

/// Reference to `libwayland-client.so`
pub(in crate::platform::linux::wayland) struct WaylandLibrary {
    /// The handle to the library containing the functions
    #[allow(unused)]
    library: SharedLibrary,

    /// The functions used by Wayland
    pub f: WaylandFunctions,
}
