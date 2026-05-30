use crate::{
    Id,
    window::{WaylandWindow, Window},
};

impl<UserEvent: 'static + Send> WaylandWindow<UserEvent> {
    /// Set the ID of the window to push events with
    pub(in crate::window::window::linux) fn set_id(&mut self, id: Id<Window<UserEvent>>) {
        self.window.set_id(id);
    }
}
