use crate::window::XdgTopLevel;
use wayland::{wl_display, wl_surface};

impl<T> XdgTopLevel<T> {
    /// Get the handle to the surface and display for this top level
    pub unsafe fn surface_and_display(&self) -> (*mut wl_surface, *mut wl_display) {
        (unsafe { self.surface.surface().handle() }, unsafe {
            self.surface.surface().connection().handle()
        })
    }
}
