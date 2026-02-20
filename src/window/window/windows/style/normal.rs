use crate::window::WindowStyle;
use win32::{
    WS_CAPTION, WS_EX_APPWINDOW, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW, WS_POPUP,
    WS_SYSMENU,
};

impl WindowStyle {
    /// Create a new [`WindowStyle`] for a normal window
    pub fn normal(bordered: bool, resizable: bool) -> WindowStyle {
        let style = match (bordered, resizable) {
            (true, true) => WS_OVERLAPPEDWINDOW,
            (true, false) => WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            (false, _) => WS_POPUP,
        };

        WindowStyle {
            style,
            ex_style: WS_EX_APPWINDOW,
        }
    }
}
