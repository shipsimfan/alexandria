use crate::platform::linux::wayland::{XdgSurface, XdgWmBase};
use std::rc::Rc;

impl<T> XdgSurface<T> {
    /// Get a reference to the contained user data
    pub fn data(&self) -> &T {
        self.listener_data
            .as_ref()
            .map(|data| unsafe { &data.as_ref().0 })
            .unwrap()
    }

    /// Get a mutable reference to the contained user data
    pub fn data_mut(&mut self) -> &mut T {
        self.listener_data
            .as_mut()
            .map(|data| unsafe { &mut data.as_mut().0 })
            .unwrap()
    }

    /// Get a pointer to the raw data
    pub(in crate::platform::linux::wayland::objects) unsafe fn raw_data(
        &self,
    ) -> *mut (T, Rc<XdgWmBase>) {
        self.listener_data.unwrap().as_ptr()
    }
}
