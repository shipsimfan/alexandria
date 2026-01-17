use crate::{WindowEvents, platform::linux::WaylandWindow};
use wayland::{wl_display, wl_surface};

impl<Callbacks: WindowEvents> WaylandWindow<Callbacks> {
    /// Get the handle for creating surfaces
    pub unsafe fn surface_creation_handle(&mut self) -> (*mut wl_display, *mut wl_surface) {
        (unsafe { self.display.handle() }, unsafe {
            self.toplevel_surface.wl_surface().handle()
        })
    }
}
