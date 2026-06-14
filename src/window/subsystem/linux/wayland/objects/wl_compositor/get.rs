use crate::window::WlCompositor;

impl WlCompositor {
    /// Get the name of this compositor in the global registry
    pub(in crate::window::subsystem::linux::wayland) fn name(&self) -> u32 {
        self.name
    }
}
