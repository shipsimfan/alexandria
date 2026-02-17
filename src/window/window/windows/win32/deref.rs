use crate::window::{Win32Window, WindowProc};
use std::ops::{Deref, DerefMut};

impl<T: WindowProc> Deref for Win32Window<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.user_data.as_ref()
    }
}

impl<T: WindowProc> DerefMut for Win32Window<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.user_data.as_mut()
    }
}
