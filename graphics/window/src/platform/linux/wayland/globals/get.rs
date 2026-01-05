use crate::platform::linux::wayland::{WaylandGlobals, WlCompositor};

impl WaylandGlobals {
    /// Get the global compositor
    pub fn compositor(&self) -> Option<&WlCompositor> {
        self.compositor.as_ref()
    }
}
