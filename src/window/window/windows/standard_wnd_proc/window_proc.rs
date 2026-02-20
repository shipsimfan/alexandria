use crate::{
    EventKind,
    window::{StandardWndProc, WindowProc},
};
use win32::{LPARAM, UINT, WM_CLOSE, WPARAM};

impl<UserEvent: 'static + Send> WindowProc for StandardWndProc<UserEvent> {
    fn wnd_proc(this: Option<&mut Self>, msg: UINT, _: WPARAM, _: LPARAM) -> bool {
        let this = match this {
            Some(this) => this,
            None => return false,
        };

        let id = match this.id {
            Some(id) => id,
            None => return false,
        };

        match msg {
            WM_CLOSE => {
                this.event_queue
                    .push(EventKind::WindowCloseRequest { id })
                    .unwrap(); // TODO: Add error handling
            }
            _ => return false,
        }

        true
    }
}
