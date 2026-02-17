use crate::window::{WindowProc, subsystem::windows::MessageOnlyWndProc};
use win32::{LPARAM, UINT, WM_DEVICECHANGE, WM_DISPLAYCHANGE, WM_DPICHANGED, WPARAM};

impl WindowProc for MessageOnlyWndProc {
    fn wnd_proc(this: Option<&mut Self>, msg: UINT, _: WPARAM, _: LPARAM) -> bool {
        let this = match this {
            Some(this) => this,
            None => return false,
        };

        match msg {
            WM_DISPLAYCHANGE | WM_DEVICECHANGE => this.enumerate_displays = true,
            WM_DPICHANGED => this.refresh_dpi = true,
            _ => return false,
        }

        true
    }
}
