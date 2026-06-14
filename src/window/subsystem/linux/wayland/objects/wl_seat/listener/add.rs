use crate::{
    Error, Result,
    window::{WlDisplay, WlSeat, WlSeatListener},
};
use std::{
    ptr::{NonNull, null_mut},
    rc::Rc,
};
use wayland::wl_seat_add_listener_dyn;

impl WlSeat<()> {
    /// Add a listener for registry events
    pub fn add_listener<T: WlSeatListener>(mut self, data: T) -> Result<WlSeat<T>> {
        let mut data = Box::new((data, self.connection.clone()));

        if unsafe {
            wl_seat_add_listener_dyn(
                self.handle,
                &WlSeat::<T>::LISTENER,
                (data.as_mut() as *mut (T, Rc<WlDisplay>)).cast(),
                *self.connection.library.f.proxy_add_listener,
            )
        } < 0
        {
            return Err(Error::new(
                "unable to set Wayland seat listener - listener already set",
            ));
        }

        let handle = self.handle;
        self.handle = null_mut();

        Ok(WlSeat {
            handle,
            name: self.name,
            listener_data: NonNull::new(Box::into_raw(data)),
            connection: self.connection.clone(),
        })
    }
}
