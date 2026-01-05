use crate::platform::linux::wayland::{WaylandGlobals, WlRegistryListener, WlRegistryRef};
use std::ffi::CStr;

impl WlRegistryListener for WaylandGlobals {
    fn global(&mut self, _: WlRegistryRef, name: u32, interface: &CStr, version: u32) {
        println!(
            "Global \"{}\" registered with name {} (version {})",
            interface.to_string_lossy(),
            name,
            version
        );
    }
}
