use crate::{
    EventKind,
    input::KeyCode,
    math::{Recti, Vector2},
    window::{StandardWndProc, WindowProc},
};
use win32::{
    HWND, IsIconic, LPARAM, MINMAXINFO, SIZE_MAXIMIZED, SIZE_MINIMIZED, UINT, WM_CLOSE,
    WM_DPICHANGED, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_GETMINMAXINFO, WM_KEYDOWN, WM_KEYUP,
    WM_KILLFOCUS, WM_MOVE, WM_SETFOCUS, WM_SHOWWINDOW, WM_SIZE, WM_SYSKEYDOWN, WM_SYSKEYUP, WPARAM,
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
                this.result = this.close();
            }
            WM_GETMINMAXINFO => {
                if this.is_fullscreen {
                    return true;
                }

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
                    this.result = this.change_rect(new_rect);
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
                        None => {
                            this.result = this.change_rect(Recti {
                                position: this.rect.position,
                                size,
                            })
                        }
                    }
                }

                let old_maximized = this.is_maximized;
                this.is_maximized = w_param & SIZE_MAXIMIZED != 0;
                if !this.is_fullscreen {
                    this.is_maximized_when_windowed = this.is_maximized;
                }
                if this.is_maximized && !old_maximized {
                    this.result = this.event_queue.push(EventKind::WindowMaximized { id });
                }

                let old_minimized = this.is_minimized;
                this.is_minimized = w_param & SIZE_MINIMIZED != 0;
                if !this.is_fullscreen {
                    this.is_minimized_when_windowed = this.is_minimized;
                }
                if this.is_minimized && !old_minimized {
                    this.result = this.event_queue.push(EventKind::WindowMinimized { id });
                }

                if !this.is_maximized && !this.is_minimized && (old_maximized || old_minimized) {
                    this.result = this.event_queue.push(EventKind::WindowRestored { id });
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
                    None => {
                        this.result = this.change_rect(Recti {
                            position,
                            size: this.rect.size,
                        })
                    }
                }
            }

            WM_SETFOCUS => {
                this.is_focused = true;
                this.result = this.event_queue.push(EventKind::WindowGainedFocus { id });
            }

            WM_KILLFOCUS => {
                this.is_focused = false;
                this.result = this.event_queue.push(EventKind::WindowLostFocus { id });
            }

            WM_SHOWWINDOW => {
                let old_visible = this.is_visible;
                this.is_visible = w_param != 0;
                if this.is_visible != old_visible {
                    if this.is_visible {
                        this.result = this.event_queue.push(EventKind::WindowShown { id });
                    } else {
                        this.result = this.event_queue.push(EventKind::WindowHidden { id });
                    }
                }
            }

            WM_DPICHANGED => {
                let new_content_scale = (w_param & 0xFFFF) as f32 / 96.0;
                let old_content_scale = this.content_scale;
                this.content_scale = new_content_scale;

                if (new_content_scale - old_content_scale).abs() > 1e-4 {
                    this.result = this.event_queue.push(EventKind::WindowContentScaleChanged {
                        id,
                        new_content_scale,
                    });
                }
            }

            WM_KEYDOWN | WM_SYSKEYDOWN => {
                let scan_code = scan_code_from_lparam(l_param);
                let key_code = keycode_from_vk(w_param, scan_code);
                let is_repeat = (l_param & (1 << 30)) != 0;
                if !is_repeat {
                    this.key_mod.apply_key_down(key_code);
                }

                this.result = this.event_queue.push(EventKind::KeyDown {
                    window_id: Some(id),
                    key_code,
                    scan_code: scan_code as _,
                    is_repeat,
                    key_mod: this.key_mod,
                });
            }
            WM_KEYUP | WM_SYSKEYUP => {
                let scan_code = scan_code_from_lparam(l_param);
                let key_code = keycode_from_vk(w_param, scan_code);
                this.key_mod.apply_key_up(key_code);

                this.result = this.event_queue.push(EventKind::KeyUp {
                    window_id: Some(id),
                    key_code,
                    scan_code: scan_code as _,
                    key_mod: this.key_mod,
                });
            }

            _ => return false,
        }

        true
    }
}

/// Get the scancode from a Windows WM_KEY* LPARAM
fn scan_code_from_lparam(l_param: LPARAM) -> u16 {
    ((l_param >> 16) & 0x1FF) as u16
}

/// Convert a Windows virtual key code to an Alexandria key code
fn keycode_from_vk(vk: WPARAM, scan_code: u16) -> KeyCode {
    match vk {
        0x08 => KeyCode::Backspace,
        0x09 => KeyCode::Tab,
        0x0D => {
            if scan_code == 0x11C {
                KeyCode::NpEnter
            } else {
                KeyCode::Enter
            }
        }
        0x1B => KeyCode::Escape,

        0x20 => KeyCode::Space,
        0xDE => KeyCode::Quote,
        0xBC => KeyCode::Comma,
        0xBD => KeyCode::Minus,
        0xBE => KeyCode::Period,
        0xBF => KeyCode::Slash,

        0x30 => KeyCode::_0,
        0x31 => KeyCode::_1,
        0x32 => KeyCode::_2,
        0x33 => KeyCode::_3,
        0x34 => KeyCode::_4,
        0x35 => KeyCode::_5,
        0x36 => KeyCode::_6,
        0x37 => KeyCode::_7,
        0x38 => KeyCode::_8,
        0x39 => KeyCode::_9,

        0xBA => KeyCode::SemiColon,
        0xBB => KeyCode::Equals,
        0xDB => KeyCode::OpenBracket,
        0xDC => KeyCode::Backslash,
        0xDD => KeyCode::CloseBracket,
        0xC0 => KeyCode::Tilde,

        0x41 => KeyCode::A,
        0x42 => KeyCode::B,
        0x43 => KeyCode::C,
        0x44 => KeyCode::D,
        0x45 => KeyCode::E,
        0x46 => KeyCode::F,
        0x47 => KeyCode::G,
        0x48 => KeyCode::H,
        0x49 => KeyCode::I,
        0x4A => KeyCode::J,
        0x4B => KeyCode::K,
        0x4C => KeyCode::L,
        0x4D => KeyCode::M,
        0x4E => KeyCode::N,
        0x4F => KeyCode::O,
        0x50 => KeyCode::P,
        0x51 => KeyCode::Q,
        0x52 => KeyCode::R,
        0x53 => KeyCode::S,
        0x54 => KeyCode::T,
        0x55 => KeyCode::U,
        0x56 => KeyCode::V,
        0x57 => KeyCode::W,
        0x58 => KeyCode::X,
        0x59 => KeyCode::Y,
        0x5A => KeyCode::Z,

        0xA0 => KeyCode::LeftShift,
        0xA1 => KeyCode::RightShift,
        0x10 => {
            if scan_code == 0x36 {
                KeyCode::RightShift
            } else {
                KeyCode::LeftShift
            }
        }
        0xA2 => KeyCode::LeftControl,
        0xA3 => KeyCode::RightControl,
        0x11 => {
            if scan_code == 0x11D {
                KeyCode::RightControl
            } else {
                KeyCode::LeftControl
            }
        }
        0xA4 => KeyCode::LeftAlt,
        0xA5 => KeyCode::RightAlt,
        0x12 => {
            if scan_code == 0x138 {
                KeyCode::RightAlt
            } else {
                KeyCode::LeftAlt
            }
        }
        0x5B => KeyCode::LeftSuper,
        0x5C => KeyCode::RightSuper,

        0x25 => KeyCode::Left,
        0x26 => KeyCode::Up,
        0x27 => KeyCode::Right,
        0x28 => KeyCode::Down,

        0x70 => KeyCode::F1,
        0x71 => KeyCode::F2,
        0x72 => KeyCode::F3,
        0x73 => KeyCode::F4,
        0x74 => KeyCode::F5,
        0x75 => KeyCode::F6,
        0x76 => KeyCode::F7,
        0x77 => KeyCode::F8,
        0x78 => KeyCode::F9,
        0x79 => KeyCode::F10,
        0x7A => KeyCode::F11,
        0x7B => KeyCode::F12,

        0x14 => KeyCode::CapsLock,
        0x91 => KeyCode::ScrollLock,
        0x90 => KeyCode::NumLock,

        0x2C => KeyCode::PrintScreen,
        0x13 => KeyCode::Pause,
        0x2D => KeyCode::Insert,
        0x2E => KeyCode::Delete,
        0x24 => KeyCode::Home,
        0x23 => KeyCode::End,
        0x21 => KeyCode::PageUp,
        0x22 => KeyCode::PageDown,

        0xB3 => KeyCode::MediaPlayPause,
        0xB2 => KeyCode::MediaStop,
        0xB0 => KeyCode::MediaNextTrack,
        0xB1 => KeyCode::MediaPreviousTrack,

        0xAD => KeyCode::VolumeMute,
        0xAE => KeyCode::VolumeDown,
        0xAF => KeyCode::VolumeUp,

        0x60 => KeyCode::Np0,
        0x61 => KeyCode::Np1,
        0x62 => KeyCode::Np2,
        0x63 => KeyCode::Np3,
        0x64 => KeyCode::Np4,
        0x65 => KeyCode::Np5,
        0x66 => KeyCode::Np6,
        0x67 => KeyCode::Np7,
        0x68 => KeyCode::Np8,
        0x69 => KeyCode::Np9,
        0x6E => KeyCode::NpDecimal,
        0x6F => KeyCode::NpDivide,
        0x6A => KeyCode::NpMultiply,
        0x6D => KeyCode::NpSubtract,
        0x6B => KeyCode::NpAdd,

        0x5D => KeyCode::Menu,

        _ => {
            #[cfg(debug_assertions)]
            println!("unknown virtual key code: {}", vk);
            KeyCode::Unknown
        }
    }
}
