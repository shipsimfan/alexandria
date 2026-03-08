use crate::{
    EventKind, Id, Result,
    window::{subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Destroy the window with the given `id`
    pub(in crate::window::subsystem) fn destroy_window(
        &mut self,
        id: Id<WindowInner<UserEvent>>,
    ) -> Result<()> {
        if self.windows.remove(id).is_some() {
            self.event_queue.push(EventKind::WindowDestroyed {
                id: unsafe { id.cast() },
            })?;
        }

        Ok(())
    }
}
