use crate::{Window, WindowState};
use std::ops::Deref;

impl Deref for Window {
    type Target = WindowState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl AsRef<WindowState> for Window {
    fn as_ref(&self) -> &WindowState {
        &self.state
    }
}
