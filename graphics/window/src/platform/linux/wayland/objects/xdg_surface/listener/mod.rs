use crate::platform::linux::wayland::XdgSurfaceRef;

mod add;
mod trampolines;

/// An item which can be used at the callback to registry events
pub(in crate::platform::linux::wayland) trait XdgSurfaceListener:
    Sized
{
    /// Called when a property of the surface changes
    fn configure(&mut self, surface: XdgSurfaceRef, serial: u32);
}
