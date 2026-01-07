use crate::platform::linux::wayland::{
    WaylandGlobals, WlRegistryListener, WlRegistryRef, XdgWmBase,
};
use std::ffi::CStr;

impl WlRegistryListener for WaylandGlobals {
    fn global(&mut self, registry: WlRegistryRef, name: u32, interface: &CStr, version: u32) {
        if self.dispatch_result.is_err() {
            return;
        }

        if self.compositor.is_none() && interface == self.compositor_name {
            match registry.bind(name, version) {
                Ok(compositor) => self.compositor = Some(compositor),
                Err(error) => self.dispatch_result = Err(error),
            }
        } else if self.xdg_wm_base.is_none() && interface == self.xdg_wm_base_name {
            match registry
                .bind::<XdgWmBase>(name, version)
                .and_then(|xdg_wm_base| xdg_wm_base.register_ping_handler().map(|_| xdg_wm_base))
            {
                Ok(xdg_wm_base) => self.xdg_wm_base = Some(xdg_wm_base),
                Err(error) => self.dispatch_result = Err(error),
            }
        }
    }
}
