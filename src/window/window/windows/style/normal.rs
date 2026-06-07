use crate::window::WindowStyle;
use win32::{
    WS_CAPTION, WS_EX_APPWINDOW, WS_MAXIMIZE, WS_MINIMIZE, WS_MINIMIZEBOX, WS_OVERLAPPED,
    WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SYSMENU, WS_VISIBLE,
};

impl WindowStyle {
    /// Create a new [`WindowStyle`] for a normal window
    pub fn normal(
        bordered: bool,
        resizable: bool,
        maximize: bool,
        minimize: bool,
        show: bool,
    ) -> WindowStyle {
        let mut style = match (bordered, resizable) {
            (true, true) => WS_OVERLAPPEDWINDOW,
            (true, false) => WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            (false, _) => WS_POPUP,
        };

        if maximize {
            style |= WS_MAXIMIZE;
        }

        if minimize {
            style |= WS_MINIMIZE;
        }

        if show {
            style |= WS_VISIBLE;
        }

        WindowStyle {
            style,
            ex_style: WS_EX_APPWINDOW,
        }
    }
}
