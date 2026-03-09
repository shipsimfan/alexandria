use crate::{
    PackedMap,
    window::{subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Get the set of currently active windows
    pub fn windows(&self) -> &PackedMap<WindowInner<UserEvent>> {
        &self.windows
    }

    /// Get the set of currently active windows mutably
    pub fn windows_mut(&mut self) -> &mut PackedMap<WindowInner<UserEvent>> {
        &mut self.windows
    }
}
