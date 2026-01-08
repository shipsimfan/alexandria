use crate::platform::linux::wayland::XdgToplevelRef;

mod trampolines;

/// An item which can be used at the callback to XDG toplevel surface events
pub(in crate::platform::linux::wayland) trait XdgToplevelListener:
    Sized
{
    /// Called when a property of the toplevel surface changes
    fn configure(&mut self, toplevel: XdgToplevelRef, width: i32, height: i32);

    /// Called when a close is requested on the toplevel surface
    fn close(&mut self, toplevel: XdgToplevelRef);

    /// Called to inform of the maximum bounds for a window before `configure`
    fn configure_bounds(&mut self, toplevel: XdgToplevelRef, width: i32, height: i32);
}
