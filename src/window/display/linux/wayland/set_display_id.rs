use crate::{
    Id,
    window::{Display, display::linux::WaylandDisplay},
};

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Set the id of this display
    pub fn set_display_id(&mut self, id: Id<Display<'static, UserEvent>>) {
        self.output.data_mut().set_display_id(id);
    }
}
