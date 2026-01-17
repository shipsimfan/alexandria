use crate::{Result, WindowSurface, instance::inner::GraphicsInstanceInner};
use alexandria_window::{Window, WindowEvents};
use std::sync::Arc;

mod wayland;

impl WindowSurface {
    /// Create a new [`WindowSurface`]
    pub(in crate::instance) fn new<Callbacks: WindowEvents>(
        instance: Arc<GraphicsInstanceInner>,
        window: &mut Window<Callbacks>,
    ) -> Result<WindowSurface> {
        match window {
            Window::Wayland(window) => WindowSurface::new_wayland(instance, window),
            Window::X11(_) => todo!(),
        }
    }
}
