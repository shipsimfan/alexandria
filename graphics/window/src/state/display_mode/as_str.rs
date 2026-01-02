use crate::DisplayMode;

impl DisplayMode {
    /// Get a string representing this display mode
    pub fn as_str(&self) -> &'static str {
        match self {
            DisplayMode::Resizable => "resizable",
            DisplayMode::Windowed => "windowed",
            DisplayMode::Borderless => "borderless",
        }
    }
}
