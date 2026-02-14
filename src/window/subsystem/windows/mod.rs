use crate::{PackedMap, window::display::DisplayInner};

mod displays;
mod new;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Winodws
pub(in crate::window) struct WindowSubsystemInner {
    /// The current set of displays
    displays: PackedMap<DisplayInner>,
}
