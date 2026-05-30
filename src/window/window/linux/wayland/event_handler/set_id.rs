use crate::{
    Id,
    window::{Window, window::linux::wayland::WaylandEventHandler},
};

impl<UserEvent: 'static + Send> WaylandEventHandler<UserEvent> {
    /// Set the ID of the window to push events with
    pub fn set_id(&mut self, id: Id<Window<UserEvent>>) {
        self.id = Some(id);
    }
}
