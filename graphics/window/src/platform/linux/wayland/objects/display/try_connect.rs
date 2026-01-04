use crate::platform::linux::wayland::{WaylandLibrary, WlDisplay};
use std::ptr::{null, null_mut};

impl WlDisplay {
    /// Attempt to connect to Wayland
    pub fn try_connect() -> Option<WlDisplay> {
        let library = WaylandLibrary::try_open()?;

        let handle = unsafe { (library.f.display_connect)(null()) };
        if handle == null_mut() {
            None
        } else {
            Some(WlDisplay { handle, library })
        }
    }
}
