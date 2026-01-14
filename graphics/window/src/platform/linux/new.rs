use crate::{
    DisplayMode, Result, Window, WindowEvents,
    platform::linux::{WaylandWindow, wayland::WlDisplay},
};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl<Callbacks: WindowEvents> Window<Callbacks> {
    /// Create a new [`Window`]
    pub(crate) fn new(
        title: Cow<'static, str>,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
        callbacks: Callbacks,
        force_x11: bool,
    ) -> Result<Box<Window<Callbacks>>> {
        // Try to connect to Wayland
        if !force_x11 {
            if let Some(display) = WlDisplay::try_connect() {
                return WaylandWindow::new(title, size, display_mode, display, callbacks);
            }
        }

        println!("Using X11!");

        todo!()
    }
}
