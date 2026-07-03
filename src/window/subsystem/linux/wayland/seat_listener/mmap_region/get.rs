use crate::window::subsystem::linux::wayland::seat_listener::MMapRegion;

impl MMapRegion {
    /// Get the contents of the memory region as a byte slice
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }
}
