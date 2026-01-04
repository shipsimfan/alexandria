use crate::{Result, Window, WindowBuilder};

impl WindowBuilder {
    /// Build a new [`Window`] with the provided properties
    pub fn build(&mut self) -> Result<Box<Window>> {
        Window::new(
            self.title.clone(),
            self.size,
            self.display_mode,
            #[cfg(target_os = "linux")]
            self.force_x11,
        )
    }
}
