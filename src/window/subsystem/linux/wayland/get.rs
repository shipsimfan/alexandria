use crate::{
    PackedMap,
    window::{
        display::DisplayInner, subsystem::linux::WaylandWindowSubsystem, window::WindowInner,
    },
};

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Get the set of currently active windows
    pub(in crate::window::subsystem::linux) fn windows(
        &self,
    ) -> &PackedMap<WindowInner<UserEvent>> {
        &self.windows
    }

    /// Get the set of currently active displays
    pub(in crate::window::subsystem::linux) fn displays(
        &self,
    ) -> &PackedMap<DisplayInner<UserEvent>> {
        self.registry.displays()
    }
}
