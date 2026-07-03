use crate::window::subsystem::linux::wayland::seat_listener::MMapRegion;
use linux::sys::mman::munmap;

impl Drop for MMapRegion {
    fn drop(&mut self) {
        unsafe { munmap(self.ptr.cast(), self.size) };
    }
}
