use crate::{
    Result,
    window::{Window, WindowBuilder, subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Create a new [`Window`] with the settings of the given builder
    pub(in crate::window::subsystem) fn create_window(
        &mut self,
        builder: &WindowBuilder<UserEvent>,
    ) -> Result<Window<UserEvent>> {
        let inner = WindowInner::new(
            self.standard_window_class.clone(),
            builder,
            &self.event_queue,
        )?;

        let id = self.windows.insert(inner);
        let cast_id = unsafe { id.cast() };

        self.windows[id].set_id(cast_id);

        Ok(Window::new(cast_id, builder.get_context().clone()))
    }
}
