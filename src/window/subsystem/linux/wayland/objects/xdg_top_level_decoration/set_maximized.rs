use crate::window::XdgTopLevelDecoration;

impl<T> XdgTopLevelDecoration<T> {
    /// Set the window to be maximized
    pub fn set_maximized(&mut self) {
        self.top_level.set_maximized();
    }
}
