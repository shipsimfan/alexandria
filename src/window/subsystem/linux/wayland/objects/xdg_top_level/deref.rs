use crate::window::XdgTopLevel;
use std::ops::{Deref, DerefMut};

impl<T> Deref for XdgTopLevel<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T> DerefMut for XdgTopLevel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}
