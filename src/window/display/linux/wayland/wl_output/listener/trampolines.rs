use crate::window::display::linux::wayland::{WlOutput, WlOutputListener};
use std::ffi::{CStr, c_char, c_void};
use wayland::{wl_output, wl_output_listener};

impl<T: WlOutputListener> WlOutput<T> {
    /// The listeners for the registry
    pub(in crate::window::display::linux::wayland::wl_output::listener) const LISTENER:
        wl_output_listener = wl_output_listener {
        geometry: geometry_trampoline::<T>,
        mode: mode_trampoline::<T>,
        done: done_trampoline::<T>,
        scale: scale_trampoline::<T>,
        name: name_trampoline::<T>,
        description: description_trampoline::<T>,
    };
}

/// Trampoline for responding to the `geometry` event of the output
unsafe extern "C" fn geometry_trampoline<T: WlOutputListener>(
    data: *mut c_void,
    _: *mut wl_output,
    x: i32,
    y: i32,
    physical_width: i32,
    physical_height: i32,
    _: i32,
    make: *const c_char,
    model: *const c_char,
    transform: i32,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    let make = unsafe { CStr::from_ptr(make) };
    let model = unsafe { CStr::from_ptr(model) };

    let transform = unsafe { std::mem::transmute(transform) };

    data.geometry(
        x,
        y,
        physical_width,
        physical_height,
        make,
        model,
        transform,
    );
}

/// Trampoline for responding to the `mode` event of the output
unsafe extern "C" fn mode_trampoline<T: WlOutputListener>(
    data: *mut c_void,
    _: *mut wl_output,
    flags: i32,
    width: i32,
    height: i32,
    refresh: i32,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    data.mode(flags, width, height, refresh);
}

/// Trampoline for responding to the `done` event of the output
unsafe extern "C" fn done_trampoline<T: WlOutputListener>(data: *mut c_void, _: *mut wl_output) {
    let data: &mut T = unsafe { &mut *data.cast() };

    data.done();
}

/// Trampoline for responding to the `scale` event of the output
unsafe extern "C" fn scale_trampoline<T: WlOutputListener>(
    data: *mut c_void,
    _: *mut wl_output,
    factor: i32,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    data.scale(factor);
}

/// Trampoline for responding to the `name` event of the output
unsafe extern "C" fn name_trampoline<T: WlOutputListener>(
    data: *mut c_void,
    _: *mut wl_output,
    name: *const c_char,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    let name = unsafe { CStr::from_ptr(name) };

    data.name(name);
}

/// Trampoline for responding to the `description` event of the output
unsafe extern "C" fn description_trampoline<T: WlOutputListener>(
    data: *mut c_void,
    _: *mut wl_output,
    description: *const c_char,
) {
    let data: &mut T = unsafe { &mut *data.cast() };

    let description = unsafe { CStr::from_ptr(description) };

    data.description(description);
}
