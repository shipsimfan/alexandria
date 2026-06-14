use crate::window::{WlDisplay, WlKeyboard, WlKeyboardListener};
use std::{ffi::c_void, rc::Rc};
use wayland::{wl_array, wl_keyboard_keymap_format, wl_keyboard_listener, wl_seat, wl_surface};

impl<T: WlKeyboardListener> WlKeyboard<T> {
    /// The listeners for the keyboard
    pub(in crate::window::subsystem::linux::wayland::objects::wl_keyboard) const LISTENER:
        wl_keyboard_listener = wl_keyboard_listener {
        keymap: keymap_trampoline::<T>,
        enter: enter_trampoline::<T>,
        leave: leave_trampoline::<T>,
        key: key_trampoline::<T>,
        modifiers: modifiers_trampoline::<T>,
        repeat_info: repeat_info_trampoline::<T>,
    };
}

/// Trampoline for responding to a keymap event from the keyboard
unsafe extern "C" fn keymap_trampoline<T: WlKeyboardListener>(
    data: *mut c_void,
    _: *mut wl_seat,
    format: u32,
    fd: i32,
    size: u32,
) {
    let data: &mut (T, Rc<WlDisplay>) = unsafe { &mut *data.cast() };

    let format = match format {
        0 => wl_keyboard_keymap_format::NoKeymap,
        1 => wl_keyboard_keymap_format::XkbV1,
        _ => return,
    };

    data.0.keymap(format, fd, size);
}

/// Trampoline for responding to an enter event from the keyboard
unsafe extern "C" fn enter_trampoline<T: WlKeyboardListener>(
    data: *mut c_void,
    seat: *mut wl_seat,
    serial: u32,
    surface: *mut wl_surface,
    keys: *mut wl_array,
) {
}

/// Trampoline for responding to a leave event from the keyboard
unsafe extern "C" fn leave_trampoline<T: WlKeyboardListener>(
    data: *mut c_void,
    seat: *mut wl_seat,
    serial: u32,
    surface: *mut wl_surface,
) {
}

/// Trampoline for responding to a key event from the keyboard
unsafe extern "C" fn key_trampoline<T: WlKeyboardListener>(
    data: *mut c_void,
    _: *mut wl_seat,
    serial: u32,
    time: u32,
    key: u32,
    state: u32,
) {
    let data: &mut (T, Rc<WlDisplay>) = unsafe { &mut *data.cast() };

    data.0.key(serial, time, key, state);
}

/// Trampoline for responding to a modifiers event from the keyboard
unsafe extern "C" fn modifiers_trampoline<T: WlKeyboardListener>(
    data: *mut c_void,
    seat: *mut wl_seat,
    serial: u32,
    mods_depressed: u32,
    mods_latched: u32,
    mods_locked: u32,
    group: u32,
) {
}

/// Trampoline for responding to a repeat info event from the keyboard
unsafe extern "C" fn repeat_info_trampoline<T: WlKeyboardListener>(
    data: *mut c_void,
    seat: *mut wl_seat,
    rate: i32,
    delay: i32,
) {
}
