use crate::{Result, WindowSurface, instance::GraphicsInstanceInner};
use alexandria_window::Window;
use std::sync::Arc;

impl GraphicsInstanceInner {
    /// Create a new [`WindowSurface`]
    pub(in crate::instance) fn create_window_surface(
        self: Arc<Self>,
        window: &mut Window,
    ) -> Result<WindowSurface> {
        WindowSurface::new(self, window)
    }
}
