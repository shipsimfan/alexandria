use crate::{
    PackedMap,
    window::{display::DisplayInner, subsystem::linux::WaylandWindowSubsystem},
};

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Get the set of currently active displays
    pub(in crate::window::subsystem::linux) fn displays(
        &self,
    ) -> &PackedMap<DisplayInner<UserEvent>> {
        self.registry.data().displays()
    }
}
