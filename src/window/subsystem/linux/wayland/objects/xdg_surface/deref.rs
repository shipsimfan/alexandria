use crate::window::XdgSurface;
use std::ops::{Deref, DerefMut};

impl<T> Deref for XdgSurface<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T> DerefMut for XdgSurface<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}
