use crate::platform::linux::{
    SharedLibrary,
    wayland::{WaylandLibrary, library::functions::WaylandFunctions},
};

impl WaylandLibrary {
    /// Attempt to open `libwayland-client.so`
    pub fn try_open() -> Option<WaylandLibrary> {
        const NAMES: &[&str] = &["libwayland-client.so.0", "libwayland-client.so"];

        for name in NAMES {
            if let Some(mut library) = SharedLibrary::try_open(name) {
                if let Some(f) = WaylandFunctions::load(&mut library) {
                    return Some(WaylandLibrary { library, f });
                }
            }
        }

        None
    }
}
