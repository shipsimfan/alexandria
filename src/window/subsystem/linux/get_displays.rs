use crate::{
    PackedMap,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner},
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Get the set of currently active displays
    pub fn displays(&self) -> &PackedMap<DisplayInner<UserEvent>> {
        match self {
            WindowSubsystemInner::Wayland(wayland) => wayland.displays(),
            WindowSubsystemInner::X11 => todo!(),
        }
    }
}
