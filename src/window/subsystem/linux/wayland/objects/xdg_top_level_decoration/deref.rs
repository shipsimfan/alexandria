use crate::window::XdgTopLevelDecoration;
use std::ops::{Deref, DerefMut};

impl<T> Deref for XdgTopLevelDecoration<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T> DerefMut for XdgTopLevelDecoration<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}
