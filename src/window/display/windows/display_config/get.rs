use crate::{math::Rational, window::display::windows::DisplayConfig};

impl DisplayConfig {
    /// Get the name of the device for this display
    pub fn gdi_name(&self) -> &[u16] {
        &self.gdi_name[..self.gdi_name_length]
    }

    /// Get a nice name reported for this display
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the best guess enumeration ID for this display
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the current refresh rate of the display
    pub fn refresh_rate(&self) -> Rational {
        self.refresh_rate
    }
}
