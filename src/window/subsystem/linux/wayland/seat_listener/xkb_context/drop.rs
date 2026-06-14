use crate::window::subsystem::linux::wayland::seat_listener::XkbContext;
use xkbcommon::xkb_context_unref;

impl Drop for XkbContext {
    fn drop(&mut self) {
        unsafe { xkb_context_unref(self.handle) };
    }
}
