use crate::{
    EventKind,
    window::{StandardWndProc, WindowProc},
};
use win32::{LPARAM, MINMAXINFO, UINT, WM_CLOSE, WM_GETMINMAXINFO, WPARAM};

impl<UserEvent: 'static + Send> WindowProc for StandardWndProc<UserEvent> {
    fn wnd_proc(this: Option<&mut Self>, msg: UINT, _: WPARAM, l_param: LPARAM) -> bool {
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
            WM_GETMINMAXINFO => {
                let min_max_info = unsafe { &mut *(l_param as *mut MINMAXINFO) };

                if let Some(minimum_size) = this.minimum_window_size {
                    min_max_info.min_track_size.x = minimum_size.x as _;
                    min_max_info.min_track_size.y = minimum_size.y as _;
                }

                if let Some(maximum_size) = this.maximum_window_size {
                    min_max_info.max_track_size.x = maximum_size.x as _;
                    min_max_info.max_track_size.y = maximum_size.y as _;
                }
            }
            _ => return false,
        }

        true
    }
}
