use crate::window::XdgTopLevelDecoration;
use std::ffi::CStr;

impl<T> XdgTopLevelDecoration<T> {
    /// Set the title of this toplevel surface
    pub(in crate::window) fn set_title(&mut self, title: &CStr) {
        self.top_level.set_title(title);
    }
}
