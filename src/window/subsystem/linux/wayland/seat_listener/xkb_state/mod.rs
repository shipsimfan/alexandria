use crate::window::subsystem::linux::wayland::seat_listener::MMapRegion;

mod drop;
mod key;
mod modifier;
mod new;

/// The state of a single xkbcommon keymap
pub(in crate::window::subsystem::linux::wayland::seat_listener) struct XkbState {
    /// The memory region containing the keymap
    _mem_region: MMapRegion,

    /// The keymap itself
    keymap: *mut xkbcommon::XkbKeymap,

    /// The state for the keymap
    state: *mut xkbcommon::XkbState,
}
