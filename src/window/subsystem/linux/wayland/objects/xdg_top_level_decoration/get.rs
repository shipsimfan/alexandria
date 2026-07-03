use crate::window::XdgTopLevelDecoration;
use wayland::{wl_display, wl_surface};

impl<T> XdgTopLevelDecoration<T> {
    /// Get the handle to the surface and display for this top level
    pub unsafe fn surface_and_display(&self) -> (*mut wl_surface, *mut wl_display) {
        (
            unsafe { self.top_level.surface().surface().handle() },
            unsafe { self.top_level.surface().surface().connection().handle() },
        )
    }
}
