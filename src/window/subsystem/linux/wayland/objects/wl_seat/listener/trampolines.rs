use crate::window::{WlDisplay, WlSeat, WlSeatListener, WlSeatRef};
use std::{
    ffi::{CStr, c_char, c_void},
    ptr::NonNull,
    rc::Rc,
};
use wayland::{wl_seat, wl_seat_listener};

impl<T: WlSeatListener> WlSeat<T> {
    /// The listeners for the seat
    pub(in crate::window::subsystem::linux::wayland::objects::wl_seat::listener) const LISTENER:
        wl_seat_listener = wl_seat_listener {
        capabilities: capabilities_trampoline::<T>,
        name: name_trampoline::<T>,
    };
}

/// Trampoline for responding to a seat capabilities event
unsafe extern "C" fn capabilities_trampoline<T: WlSeatListener>(
    data_ptr: *mut c_void,
    seat: *mut wl_seat,
    capabilities: u32,
) {
    let data: &mut (T, Rc<WlDisplay>) = unsafe { &mut *data_ptr.cast() };

    let seat = WlSeatRef::new(seat, &data.1, NonNull::new(data_ptr.cast()).unwrap());

    data.0.capabilities(seat, capabilities);
}

/// Trampoline for responding to a seat name event
unsafe extern "C" fn name_trampoline<T: WlSeatListener>(
    data_ptr: *mut c_void,
    seat: *mut wl_seat,
    name: *const c_char,
) {
    let data: &mut (T, Rc<WlDisplay>) = unsafe { &mut *data_ptr.cast() };

    let seat = WlSeatRef::new(seat, &data.1, NonNull::new(data_ptr.cast()).unwrap());

    let name = unsafe { CStr::from_ptr(name) };
    data.0.name(seat, name);
}
