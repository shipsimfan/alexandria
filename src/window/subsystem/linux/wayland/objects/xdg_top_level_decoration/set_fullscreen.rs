use crate::window::{WlOutput, XdgTopLevelDecoration};

impl<T> XdgTopLevelDecoration<T> {
    /// Set the window to be fullscreen
    pub fn set_fullscreen<T2>(&mut self, output: Option<&WlOutput<T2>>) {
        self.top_level.set_fullscreen(output);
    }
}
