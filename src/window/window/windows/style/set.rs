use crate::window::WindowStyle;
use win32::{WS_MAXIMIZE, WS_MINIMIZE, WS_VISIBLE};

impl WindowStyle {
    /// Add the maximize style to the window style
    pub fn maximize(&mut self) {
        self.style |= WS_MAXIMIZE;
    }

    /// Add the minimize style to the window style
    pub fn minimize(&mut self) {
        self.style |= WS_MINIMIZE;
    }

    /// Add the visible style to the window style
    pub fn show(&mut self) {
        self.style |= WS_VISIBLE;
    }
}
