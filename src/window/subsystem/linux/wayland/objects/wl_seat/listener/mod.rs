use crate::window::WlSeatRef;
use std::ffi::CStr;

mod add;
mod trampolines;

/// An item which can be used at the callback to Wayland seat events
pub(in crate::window) trait WlSeatListener: Sized {
    /// Called when the capabilities of a seat change
    fn capabilities(&mut self, seat: WlSeatRef<Self>, capabilities: u32);

    /// Called when the name of a seat changes
    fn name(&mut self, seat: WlSeatRef<Self>, name: &CStr);
}
