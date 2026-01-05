use crate::{
    Result, WindowError,
    platform::linux::wayland::{WlRegistry, WlRegistryListener},
};
use std::ptr::NonNull;
use wayland::wl_registry_add_listener_dyn;

impl WlRegistry {
    /// Add a listener for registry events
    pub fn add_listener<T: WlRegistryListener>(self, data: T) -> Result<WlRegistry<T>> {
        let mut data = Box::new(data);

        if unsafe {
            wl_registry_add_listener_dyn(
                self.handle,
                &WlRegistry::<T>::LISTENER,
                (data.as_mut() as *mut T).cast(),
                self.display.library.f.proxy_add_listener,
            )
        } < 0
        {
            return Err(WindowError::new(
                "unable to set registry listener - listener already set",
            ));
        }

        Ok(WlRegistry {
            handle: self.handle,
            listener_data: NonNull::new(Box::into_raw(data)),
            display: self.display.clone(),
        })
    }
}
