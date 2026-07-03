use crate::window::XdgTopLevelDecoration;

impl<T> XdgTopLevelDecoration<T> {
    /// Commit the current state of this toplevel surface to the compositor
    pub(in crate::window) fn commit(&mut self) {
        self.top_level.surface_mut().surface_mut().commit();
    }
}
