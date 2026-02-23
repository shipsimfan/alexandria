use crate::window::subsystem::linux::wayland::{WaylandGlobals, WlRegistryListener, WlRegistryRef};
use std::ffi::CStr;

impl<UserEvent: 'static + Send> WlRegistryListener for WaylandGlobals<UserEvent> {
    fn global(&mut self, mut registry: WlRegistryRef, name: u32, interface: &CStr, version: u32) {
        if self.dispatch_result.is_err() {
            return;
        }

        self.dispatch_result = self.add_global(&mut registry, name, interface, version);
    }

    fn global_remove(&mut self, _: WlRegistryRef, name: u32) {
        if self.dispatch_result.is_err() {
            return;
        }

        self.dispatch_result = self.remove_global(name);
    }
}
