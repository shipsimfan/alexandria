use crate::{
    EventKind,
    math::{Recti, Vector2},
    window::{StandardWndProc, WindowProc},
};
use win32::{
    HWND, IsIconic, LPARAM, MINMAXINFO, SIZE_MAXIMIZED, SIZE_MINIMIZED, UINT, WM_CLOSE,
    WM_DPICHANGED, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_GETMINMAXINFO, WM_KILLFOCUS, WM_MOVE,
    WM_SETFOCUS, WM_SHOWWINDOW, WM_SIZE, WPARAM,
};

impl<UserEvent: 'static + Send> WindowProc for StandardWndProc<UserEvent> {
    fn wnd_proc(
        this: Option<&mut Self>,
        wnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> bool {
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
                this.close().unwrap(); // TODO: Add error handling
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

            // The user has begun moving or resizing the window
            WM_ENTERSIZEMOVE => this.is_changing = Some(this.rect),

            // The user has stopped moving or resizing the window
            WM_EXITSIZEMOVE => {
                if let Some(new_rect) = this.is_changing.take() {
                    this.change_rect(new_rect).unwrap(); // TODO: Add error handling
                }
            }

            // The window has changed size
            WM_SIZE => {
                let width = (l_param & 0xFFFF) as _;
                let height = ((l_param >> 16) & 0xFFFF) as _;
                if width != 0 && height != 0 {
                    let size = Vector2::new(width, height);
                    match &mut this.is_changing {
                        Some(is_changing) => {
                            is_changing.size = size;
                        }
                        None => this
                            .change_rect(Recti {
                                position: this.rect.position,
                                size,
                            })
                            .unwrap(), // TODO: Add error handling
                    }
                }

                let old_maximized = this.is_maximized;
                this.is_maximized = w_param & SIZE_MAXIMIZED != 0;
                if !this.is_fullscreen {
                    this.is_maximized_when_windowed = this.is_maximized;
                }
                if this.is_maximized && !old_maximized {
                    this.event_queue
                        .push(EventKind::WindowMaximized { id })
                        .unwrap(); // TODO: Add error handling
                }

                let old_minimized = this.is_minimized;
                this.is_minimized = w_param & SIZE_MINIMIZED != 0;
                if !this.is_fullscreen {
                    this.is_minimized_when_windowed = this.is_minimized;
                }
                if this.is_minimized && !old_minimized {
                    this.event_queue
                        .push(EventKind::WindowMinimized { id })
                        .unwrap(); // TODO: Add error handling
                }

                if !this.is_maximized && !this.is_minimized && (old_maximized || old_minimized) {
                    this.event_queue
                        .push(EventKind::WindowRestored { id })
                        .unwrap(); // TODO: Add error handling
                }
            }

            WM_MOVE => {
                if unsafe { IsIconic(wnd) } != 0 {
                    return true;
                }

                let x = (l_param & 0xFFFF) as i16;
                let y = ((l_param >> 16) & 0xFFFF) as i16;
                let position = Vector2::new(x as _, y as _);
                match &mut this.is_changing {
                    Some(is_changing) => {
                        is_changing.position = position;
                    }
                    None => this
                        .change_rect(Recti {
                            position,
                            size: this.rect.size,
                        })
                        .unwrap(), // TODO: Add error handling
                }
            }

            WM_SETFOCUS => {
                this.is_focused = true;
                this.event_queue
                    .push(EventKind::WindowGainedFocus { id })
                    .unwrap(); // TODO: Add error handling
            }

            WM_KILLFOCUS => {
                this.is_focused = false;
                this.event_queue
                    .push(EventKind::WindowLostFocus { id })
                    .unwrap(); // TODO: Add error handling
            }

            WM_SHOWWINDOW => {
                let old_visible = this.is_visible;
                this.is_visible = w_param != 0;
                if this.is_visible != old_visible {
                    if this.is_visible {
                        this.event_queue
                            .push(EventKind::WindowShown { id })
                            .unwrap(); // TODO: Add error handling
                    } else {
                        this.event_queue
                            .push(EventKind::WindowHidden { id })
                            .unwrap(); // TODO: Add error handling
                    }
                }
            }

            WM_DPICHANGED => {
                let new_content_scale = (w_param & 0xFFFF) as f32 / 96.0;
                let old_content_scale = this.content_scale;
                this.content_scale = new_content_scale;

                if (new_content_scale - old_content_scale).abs() > 1e-4 {
                    this.event_queue
                        .push(EventKind::WindowContentScaleChanged {
                            id,
                            new_content_scale,
                        })
                        .unwrap(); // TODO: Add error handling
                }
            }

            _ => return false,
        }

        true
    }
}
