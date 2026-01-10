use crate::{DisplayMode, WindowBuilder, WindowEvents};
use alexandria_math::Vector2u;

impl<Callbacks: WindowEvents> WindowBuilder<Callbacks> {
    /// Get the title the window will be created with
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Get the size the window will be created at
    pub fn get_size(&self) -> Option<Vector2u> {
        self.size
    }

    /// Get the display mode the window will be created with
    pub fn get_display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Get a reference to the contained window event callback item
    pub fn get_callbacks(&self) -> &Callbacks {
        &self.callbacks
    }

    /// Get a mutable reference to the contained window event callback item
    pub fn get_callbacks_mut(&mut self) -> &mut Callbacks {
        &mut self.callbacks
    }

    /// Is X11 being forced to be used over Wayland?
    #[cfg(target_os = "linux")]
    pub fn get_force_x11(&self) -> bool {
        self.force_x11
    }
}
