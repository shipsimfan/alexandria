use crate::{
    Result,
    window::{Window, WindowBuilder, subsystem::WindowSubsystemInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Create a new [`Window`] with the settings of the given builder
    pub(in crate::window::subsystem) fn create_window(
        &mut self,
        builder: &WindowBuilder<UserEvent>,
    ) -> Result<Window<UserEvent>> {
        match self {
            WindowSubsystemInner::Wayland(wayland) => wayland.create_window(builder),
            WindowSubsystemInner::X11 => todo!(),
        }
    }
}
