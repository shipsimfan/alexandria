use crate::window::XdgTopLevel;

impl<T> XdgTopLevel<T> {
    /// Commit the current state of this toplevel surface to the compositor
    pub(in crate::window) fn commit(&mut self) {
        self.surface.surface_mut().commit();
    }
}
