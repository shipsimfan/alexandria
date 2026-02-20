use crate::{
    Id,
    window::{subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Destroy the window with the given `id`
    pub(in crate::window::subsystem) fn destroy_window(&mut self, id: Id<WindowInner<UserEvent>>) {
        self.windows.remove(id);
    }
}
