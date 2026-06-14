mod clone;
mod drop;
mod get;
mod new;

/// The main context for xkbcommon
pub(in crate::window::subsystem::linux::wayland::seat_listener) struct XkbContext {
    /// The underlying handle to the xkb context
    handle: *mut xkbcommon::XkbContext,
}
