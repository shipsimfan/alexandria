use crate::WindowSurface;
use std::ptr::null;

impl Drop for WindowSurface {
    fn drop(&mut self) {
        (self.instance.functions.surface().destroy_surface)(
            self.instance.handle(),
            self.handle,
            null(),
        )
    }
}
