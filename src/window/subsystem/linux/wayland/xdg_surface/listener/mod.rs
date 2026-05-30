use crate::window::XdgSurfaceRef;

mod add;
mod trampolines;

/// An item which can be used at the callback to XDG surface events
pub(in crate::window) trait XdgSurfaceListener: Sized {
    /// Called when a property of the surface changes
    fn configure(&mut self, surface: XdgSurfaceRef, serial: u32);
}
