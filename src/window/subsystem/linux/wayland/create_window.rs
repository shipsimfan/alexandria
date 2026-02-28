use crate::{
    Result,
    window::{Window, WindowBuilder, subsystem::linux::wayland::WaylandWindowSubsystem},
};

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Create a new [`Window`] with the settings of the given builder
    pub(in crate::window::subsystem::linux) fn create_window(
        &mut self,
        builder: &WindowBuilder<UserEvent>,
    ) -> Result<Window<UserEvent>> {
        todo!()
    }
}
