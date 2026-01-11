use crate::{GraphicsInstance, Result, WindowSurface};
use alexandria_window::Window;

impl GraphicsInstance {
    /// Create a new [`WindowSurface`]
    pub fn create_window_surface(&self, window: &mut Window) -> Result<WindowSurface> {
        self.inner.clone().create_window_surface(window)
    }
}
