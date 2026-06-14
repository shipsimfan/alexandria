use crate::window::XdgTopLevelDecoration;

impl<T> XdgTopLevelDecoration<T> {
    /// Set the window to be minimized
    pub fn set_minimized(&mut self) {
        self.top_level.set_minimized();
    }
}
