use crate::window::{WlOutput, XdgOutput};

impl<T> XdgOutput<T> {
    /// Downgrades this to a [`WlOutput`]
    pub(in crate::window::display::linux::wayland) fn downgrade(&mut self) -> WlOutput<T> {
        self.wl_output.take().unwrap()
    }
}
