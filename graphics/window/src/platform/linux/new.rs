use crate::{DisplayMode, Result, Window, platform::linux::wayland::WlDisplay};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl Window {
    /// Create a new [`Window`]
    pub(crate) fn new(
        title: Cow<'static, str>,
        size: Option<Vector2u>,
        display_mode: DisplayMode,
        force_x11: bool,
    ) -> Result<Box<Window>> {
        // Try to connect to Wayland
        if !force_x11 {
            if let Some(wl_display) = WlDisplay::try_connect() {
                println!("Using Wayland!");

                drop(wl_display);

                todo!()
            }
        }

        println!("Using X11!");

        todo!()
    }
}
