use crate::window::{WindowProc, window::windows::StandardWndProc};
use win32::{LPARAM, UINT, WPARAM};

impl WindowProc for StandardWndProc {
    fn wnd_proc(this: Option<&mut Self>, msg: UINT, _: WPARAM, _: LPARAM) -> bool {
        let this = match this {
            Some(this) => this,
            None => return false,
        };

        match msg {
            _ => return false,
        }

        true
    }
}
