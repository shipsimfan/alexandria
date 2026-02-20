use crate::window::WindowStyle;
use win32::{WS_EX_APPWINDOW, WS_POPUP, WS_VISIBLE};

impl WindowStyle {
    /// Create the window style for a fullscreen window
    pub fn fullscreen() -> WindowStyle {
        WindowStyle {
            style: WS_POPUP | WS_VISIBLE,
            ex_style: WS_EX_APPWINDOW,
        }
    }
}
