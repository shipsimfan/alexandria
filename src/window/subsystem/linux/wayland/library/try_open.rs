use crate::{
    Error, Result, SharedObject,
    window::subsystem::linux::wayland::{WaylandFunctions, WaylandLibrary},
};

impl WaylandLibrary {
    /// Attempt to open `libwayland-client.so`
    pub fn try_open() -> Result<WaylandLibrary> {
        const NAMES: &[&str] = &["libwayland-client.so.0", "libwayland-client.so"];

        let mut result = Err(Error::new("no wayland client libraries configured"));
        for name in NAMES {
            let mut library = match SharedObject::open(name) {
                Ok(library) => library,
                Err(error) => {
                    result = Err(error);
                    continue;
                }
            };

            match WaylandFunctions::load(&mut library) {
                Ok(f) => return Ok(WaylandLibrary { library, f }),
                Err(error) => {
                    result = Err(error);
                    continue;
                }
            }
        }

        result
    }
}
