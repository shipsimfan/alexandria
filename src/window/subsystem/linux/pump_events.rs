use crate::{Result, window::subsystem::WindowSubsystemInner};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Pumps events from input devices and the window system onto the event queue
    pub(in crate::window::subsystem) fn pump_events(&mut self) -> Result<()> {
        match self {
            WindowSubsystemInner::Wayland(wayland) => wayland.pump_events(),
            WindowSubsystemInner::X11 => todo!(),
        }
    }
}
