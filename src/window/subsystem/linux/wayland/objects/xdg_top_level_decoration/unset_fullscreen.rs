use crate::window::XdgTopLevelDecoration;

impl<T> XdgTopLevelDecoration<T> {
    /// Unset the fullscreen state of the window
    pub fn unset_fullscreen(&mut self) {
        self.top_level.unset_fullscreen();
    }
}
