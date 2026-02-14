use crate::{
    PackedMap,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner},
};

impl WindowSubsystemInner {
    /// Get the set of currently active displays
    pub(in crate::window) fn displays(&self) -> &PackedMap<DisplayInner> {
        &self.displays
    }
}
