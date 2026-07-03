use crate::window::subsystem::linux::wayland::WlRegistry;
use std::ops::{Deref, DerefMut};

impl<T> Deref for WlRegistry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T> DerefMut for WlRegistry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}
