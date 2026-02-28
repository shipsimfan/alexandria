use crate::{
    Id,
    window::{Display, display::linux::WaylandDisplay},
};

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Set the id of this display
    pub(in crate::window::display::linux) fn set_display_id(
        &mut self,
        id: Id<Display<'static, UserEvent>>,
    ) {
        self.data_mut().set_display_id(id);
    }
}
