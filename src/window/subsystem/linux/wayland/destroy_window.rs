use crate::{
    Id,
    window::{subsystem::linux::wayland::WaylandWindowSubsystem, window::WindowInner},
};

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Destroy the window with the given `id`
    pub(in crate::window::subsystem::linux) fn destroy_window(
        &mut self,
        id: Id<WindowInner<UserEvent>>,
    ) {
        todo!()
    }
}
