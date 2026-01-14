use crate::{
    WindowEvents,
    platform::linux::wayland::{WaylandEventHandler, XdgSurfaceListener, XdgSurfaceRef},
};

impl<Callbacks: WindowEvents> XdgSurfaceListener for WaylandEventHandler<Callbacks> {
    fn configure(&mut self, mut surface: XdgSurfaceRef, serial: u32) {
        surface.ack_configure(serial);

        surface.set_window_geometry(0, 0, self.state.width() as _, self.state.height() as _);

        if self.did_close_request {
            self.did_close_request = false;
            self.callbacks.on_close_requested();
        }

        if self.did_resize {
            self.did_resize = false;
            self.callbacks.on_resize(self.state.size());
        }

        if self.did_maximize_or_restore {
            self.did_maximize_or_restore = false;
            if self.state.is_maximized() {
                self.callbacks.on_maximized();
            } else {
                self.callbacks.on_restored();
            }
        }
    }
}
