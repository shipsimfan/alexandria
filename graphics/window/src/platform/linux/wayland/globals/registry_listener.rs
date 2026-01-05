use crate::platform::linux::wayland::{WaylandGlobals, WlRegistryListener, WlRegistryRef};
use std::ffi::CStr;

impl WlRegistryListener for WaylandGlobals {
    fn global(&mut self, registry: WlRegistryRef, name: u32, interface: &CStr, version: u32) {
        if self.dispatch_result.is_err() {
            return;
        }

        if interface == self.compositor_name {
            println!("Found compositor at name {} (v{})", name, version);

            match registry.bind(name, version) {
                Ok(compositor) => self.compositor = Some(compositor),
                Err(error) => self.dispatch_result = Err(error),
            }
        }
    }
}
