use crate::{
    PackedMap,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Get the set of currently active windows
    pub fn windows(&self) -> &PackedMap<WindowInner<UserEvent>> {
        match self {
            WindowSubsystemInner::Wayland(subsystem) => subsystem.windows(),
            WindowSubsystemInner::X11 => todo!(),
        }
    }

    /// Get the set of currently active windows mutably and the set of displays
    pub fn windows_mut_and_displays(
        &mut self,
    ) -> (
        &mut PackedMap<WindowInner<UserEvent>>,
        &PackedMap<DisplayInner<UserEvent>>,
    ) {
        todo!()
    }
}
