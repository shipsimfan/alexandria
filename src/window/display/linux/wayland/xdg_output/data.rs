use crate::window::XdgOutput;

impl<T> XdgOutput<T> {
    /// Get a reference to the contained user data
    pub(in crate::window::display::linux::wayland) fn data(&self) -> &T {
        self.wl_output().data()
    }

    /// Get a mutable reference to the contained user data
    pub(in crate::window::display::linux::wayland) fn data_mut(&mut self) -> &mut T {
        self.wl_output_mut().data_mut()
    }
}
