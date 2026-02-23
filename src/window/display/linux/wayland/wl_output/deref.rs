use crate::window::display::linux::wayland::WlOutput;
use std::ops::{Deref, DerefMut};

impl<T> Deref for WlOutput<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T> DerefMut for WlOutput<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}
