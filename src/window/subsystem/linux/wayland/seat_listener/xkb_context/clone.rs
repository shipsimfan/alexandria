use crate::window::subsystem::linux::wayland::seat_listener::XkbContext;
use xkbcommon::xkb_context_ref;

impl Clone for XkbContext {
    fn clone(&self) -> Self {
        let handle = unsafe { xkb_context_ref(self.handle) };
        XkbContext { handle }
    }
}
