use crate::window::display::DisplayInner;

// rustdoc imports
#[allow(unused_imports)]
use crate::window::display::linux::wayland::{WlOutput, XdgOutput};

impl<UserEvent: 'static + Send> DisplayInner<UserEvent> {
    /// Update this display from a [`XdgOutput`] to a [`WlOutput`], if it is currently a [`XdgOutput`].
    pub(in crate::window) fn wayland_downgrade(&mut self) {
        match self {
            DisplayInner::Wayland(wayland) => wayland.downgrade(),
            DisplayInner::X11 => {}
        }
    }
}
