use crate::{
    PackedMap,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Get the set of currently active displays
    pub fn displays(&self) -> &PackedMap<DisplayInner<UserEvent>> {
        &self.displays
    }
}
