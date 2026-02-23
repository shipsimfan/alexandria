use crate::{
    Id,
    window::{Display, display::DisplayInner},
};

impl<UserEvent: 'static + Send> DisplayInner<UserEvent> {
    /// Set the id of this display
    pub fn set_display_id(&mut self, id: Id<Display<'static, UserEvent>>) {
        match self {
            DisplayInner::Wayland(display) => display.set_display_id(id),
            DisplayInner::X11 => todo!(),
        }
    }
}
