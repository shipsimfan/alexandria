use crate::window::WlSeat;

impl<T> WlSeat<T> {
    /// Get the name of this seat in the global registry
    pub(in crate::window::subsystem::linux::wayland) fn name(&self) -> u32 {
        self.name
    }
}
