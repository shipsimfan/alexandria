use crate::{
    Id,
    window::{Display, display::linux::wayland::WaylandDisplayEventHandler},
};

impl<UserEvent: 'static + Send> WaylandDisplayEventHandler<UserEvent> {
    /// Set the id of this display
    pub fn set_display_id(&mut self, id: Id<Display<'static, UserEvent>>) {
        self.display_id = Some(id);
    }
}
