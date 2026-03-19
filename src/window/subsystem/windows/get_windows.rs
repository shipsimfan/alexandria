use crate::{
    PackedMap,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Get the set of currently active windows
    pub fn windows(&self) -> &PackedMap<WindowInner<UserEvent>> {
        &self.windows
    }

    /// Get the set of currently active windows mutably and the set of displays
    pub fn windows_mut_and_displays(
        &mut self,
    ) -> (
        &mut PackedMap<WindowInner<UserEvent>>,
        &PackedMap<DisplayInner<UserEvent>>,
    ) {
        (&mut self.windows, &self.displays)
    }
}
