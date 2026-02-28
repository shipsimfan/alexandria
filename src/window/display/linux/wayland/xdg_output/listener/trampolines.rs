use crate::window::{XdgOutput, XdgOutputListener};
use std::ffi::{CStr, c_char, c_void};
use wayland::xdg_output::{zxdg_output_v1, zxdg_output_v1_listener};

#[allow(private_bounds)]
impl<T: XdgOutputListener> XdgOutput<T> {
    /// The listeners for the registry
    pub(in crate::window::display::linux::wayland::xdg_output) const LISTENER:
        zxdg_output_v1_listener = zxdg_output_v1_listener {
        logical_position: logical_position_trampoline::<T>,
        logical_size: logical_size_trampoline::<T>,
        done: done_trampoline::<T>,
        name: name_trampoline::<T>,
        description: description_trampoline::<T>,
    };
}

/// Trampoline for responding to the `logical_position` event of the XDG output
unsafe extern "C" fn logical_position_trampoline<T: XdgOutputListener>(
    data: *mut c_void,
    _: *mut zxdg_output_v1,
    x: i32,
    y: i32,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    data.logical_position(x, y);
}

/// Trampoline for responding to the `logical_size` event of the XDG output
unsafe extern "C" fn logical_size_trampoline<T: XdgOutputListener>(
    data: *mut c_void,
    _: *mut zxdg_output_v1,
    width: i32,
    height: i32,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    data.logical_size(width, height);
}

/// Trampoline for responding to the `done` event of the XDG output
unsafe extern "C" fn done_trampoline<T: XdgOutputListener>(_: *mut c_void, _: *mut zxdg_output_v1) {
}

/// Trampoline for responding to the `name` event of the XDG output
unsafe extern "C" fn name_trampoline<T: XdgOutputListener>(
    data: *mut c_void,
    _: *mut zxdg_output_v1,
    name: *const c_char,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    let name = unsafe { CStr::from_ptr(name) };

    data.name(name);
}

/// Trampoline for responding to the `description` event of the XDG output
unsafe extern "C" fn description_trampoline<T: XdgOutputListener>(
    data: *mut c_void,
    _: *mut zxdg_output_v1,
    description: *const c_char,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    let description = unsafe { CStr::from_ptr(description) };

    data.description(description);
}
