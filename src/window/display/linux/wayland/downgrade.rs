use crate::window::display::linux::wayland::WaylandDisplay;

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Downgrades the display to a [`WlOutput`] if it is currently an [`XdgOutput`]
    pub(in crate::window) fn downgrade(&mut self) {
        if let WaylandDisplay::Xdg(xdg_output) = self {
            *self = WaylandDisplay::Wl(xdg_output.downgrade());
        }
    }
}
