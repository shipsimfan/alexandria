use crate::{
    WindowState,
    platform::linux::wayland::{XdgSurfaceListener, XdgSurfaceRef},
};

impl XdgSurfaceListener for WindowState {
    fn configure(&mut self, mut surface: XdgSurfaceRef, serial: u32) {
        surface.ack_configure(serial);
    }
}
