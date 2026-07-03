use crate::{
    Error, Result,
    window::{WlDisplay, WlKeyboard, WlKeyboardListener},
};
use std::{marker::PhantomData, ptr::NonNull, rc::Rc};
use wayland::{wl_keyboard, wl_keyboard_add_listener_dyn};

impl<T: WlKeyboardListener> WlKeyboard<T> {
    /// Create a new [`WlKeyboard`]
    pub(in crate::window::subsystem::linux::wayland) fn new(
        handle: *mut wl_keyboard,
        connection: Rc<WlDisplay>,
        user_data: NonNull<(T, Rc<WlDisplay>)>,
    ) -> Result<WlKeyboard<T>> {
        if unsafe {
            wl_keyboard_add_listener_dyn(
                handle,
                &WlKeyboard::<T>::LISTENER,
                user_data.as_ptr().cast(),
                *connection.library.f.proxy_add_listener,
            )
        } < 0
        {
            return Err(Error::new(
                "unable to set Wayland keyboard listener - listener already set",
            ));
        }

        Ok(WlKeyboard {
            handle,
            connection,
            _phantom: PhantomData,
        })
    }
}
