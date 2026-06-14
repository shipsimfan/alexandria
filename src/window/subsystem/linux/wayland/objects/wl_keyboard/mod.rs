use crate::window::WlDisplay;
use std::{marker::PhantomData, rc::Rc};
use wayland::wl_keyboard;

mod listener;

mod drop;
mod new;

pub(in crate::window) use listener::WlKeyboardListener;

/// A keyboard input device
pub(in crate::window) struct WlKeyboard<T> {
    /// The handle to the underlying Wayland keyboard
    handle: *mut wl_keyboard,

    /// The display this keyboard came from
    connection: Rc<WlDisplay>,

    /// A marker for the listener type
    _phantom: PhantomData<T>,
}
