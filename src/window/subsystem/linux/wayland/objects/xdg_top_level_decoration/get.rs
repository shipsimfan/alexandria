use crate::window::{XdgTopLevel, XdgTopLevelDecoration};

impl<T> XdgTopLevelDecoration<T> {
    /// Get a reference to the [`XdgTopLevel`] for this top level decoration
    pub fn top_level(&self) -> &XdgTopLevel<T> {
        &self.top_level
    }

    /// Get a mutable reference to the [`XdgTopLevel`] for this top level decoration
    pub fn top_level_mut(&mut self) -> &mut XdgTopLevel<T> {
        &mut self.top_level
    }
}
