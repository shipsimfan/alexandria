use crate::{
    PackedMap,
    window::{subsystem::WindowSubsystemInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Get the set of currently active windows
    pub fn windows(&self) -> &PackedMap<WindowInner<UserEvent>> {
        todo!()
    }
}
