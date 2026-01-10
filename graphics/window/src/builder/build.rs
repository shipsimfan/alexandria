use crate::{Result, Window, WindowBuilder, WindowEvents};

impl<Callbacks: WindowEvents> WindowBuilder<Callbacks> {
    /// Build a new [`Window`] with the provided properties
    pub fn build(self) -> Result<Box<Window<Callbacks>>> {
        Window::new(
            self.title,
            self.size,
            self.display_mode,
            self.callbacks,
            #[cfg(target_os = "linux")]
            self.force_x11,
        )
    }
}
