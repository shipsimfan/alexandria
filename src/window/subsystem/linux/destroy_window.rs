use crate::{
    Id,
    window::{subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Destroy the window with the given `id`
    pub(in crate::window::subsystem) fn destroy_window(&mut self, id: Id<WindowInner<UserEvent>>) {
        match self {
            WindowSubsystemInner::Wayland(wayland) => wayland.destroy_window(id),
            WindowSubsystemInner::X11 => todo!(),
        }
    }
}
