use crate::{Window, WindowEvents, WindowState};
use std::ops::Deref;

impl<Callbacks: WindowEvents> Deref for Window<Callbacks> {
    type Target = WindowState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<Callbacks: WindowEvents> AsRef<WindowState> for Window<Callbacks> {
    fn as_ref(&self) -> &WindowState {
        &self.state
    }
}
