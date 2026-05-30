use crate::{
    Result,
    window::{
        WaylandWindow, Window, WindowBuilder, subsystem::linux::wayland::WaylandWindowSubsystem,
    },
};

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Create a new [`Window`] with the settings of the given builder
    pub(in crate::window::subsystem::linux) fn create_window(
        &mut self,
        builder: &WindowBuilder<UserEvent>,
    ) -> Result<Window<UserEvent>> {
        let inner = WaylandWindow::new(builder, &self.event_queue, &mut self.registry)?;

        let id = self.windows.insert(inner);
        let cast_id = unsafe { id.cast() };

        self.windows[id].set_id(cast_id);

        self.connection.roundtrip()?;

        Ok(Window::new(cast_id, builder.get_context().clone()))
    }
}
