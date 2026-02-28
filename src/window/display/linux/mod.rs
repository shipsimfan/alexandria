use wayland::WaylandDisplay;

mod wayland;

mod get;
mod new_wayland;
mod set_display_id;
mod wayland_downgrade;
mod wayland_upgrade;

pub(in crate::window) use wayland::{WlOutput, XdgOutput, XdgOutputListener};

/// The implementation of [`Display`](crate::window::Display)s for Linux
pub(in crate::window) enum DisplayInner<UserEvent: 'static + Send> {
    /// The Wayland implementation of a display
    Wayland(WaylandDisplay<UserEvent>),

    /// The X11 implementation of a display
    X11,
}
