use crate::{
    Error, Result,
    window::{WlOutput, display::linux::wayland::WlOutputListener},
};
use std::ptr::NonNull;
use wayland::wl_output_add_listener_dyn;

impl WlOutput {
    /// Add a listener for output events
    pub(in crate::window::display::linux::wayland) fn add_listener<T: WlOutputListener>(
        mut self,
        data: T,
    ) -> Result<WlOutput<T>> {
        let mut data = Box::new(data);

        if unsafe {
            wl_output_add_listener_dyn(
                self.handle,
                &WlOutput::<T>::LISTENER,
                (data.as_mut() as *mut T).cast(),
                *self.connection.library.f.proxy_add_listener,
            )
        } < 0
        {
            return Err(Error::new(
                "unable to set output listener - listener already set",
            ));
        }

        self.drop = false;

        Ok(WlOutput {
            handle: self.handle,
            name: self.name,
            drop: true,
            listener_data: NonNull::new(Box::into_raw(data)),
            connection: self.connection.clone(),
        })
    }
}
