use crate::{
    Id,
    window::{Window, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the ID of the window to push events with
    pub(in crate::window) fn set_id(&mut self, id: Id<Window<UserEvent>>) {
        match self {
            WindowInner::Wayland(window) => window.set_id(id),
            WindowInner::X11 => todo!(),
        }
    }
}
