use crate::{
    Result, WindowError,
    platform::linux::wayland::{WlDisplay, XdgWmBase},
};
use std::{ffi::c_void, rc::Rc};
use wayland::xdg_shell::{
    xdg_wm_base, xdg_wm_base_add_listener_dyn, xdg_wm_base_listener, xdg_wm_base_pong_dyn,
};

/// Respond to an `xdg_wm_base` ping
unsafe extern "C" fn ping_handler(data: *mut c_void, wm_base: *mut xdg_wm_base, serial: u32) {
    let display: &WlDisplay = unsafe { &*data.cast() };

    unsafe {
        xdg_wm_base_pong_dyn(
            wm_base,
            serial,
            display.library.f.proxy_marshal_flags,
            display.library.f.proxy_get_version,
        )
    };
}

/// The listener for [`xdg_wm_base`] events
const LISTENER: xdg_wm_base_listener = xdg_wm_base_listener { ping: ping_handler };

impl XdgWmBase {
    /// Register the handler function for XDG window manager ping
    pub fn register_ping_handler(&self) -> Result<()> {
        if unsafe {
            xdg_wm_base_add_listener_dyn(
                self.handle,
                &LISTENER,
                Rc::as_ptr(&self.display).cast_mut().cast(),
                self.display.library.f.proxy_add_listener,
            )
        } < 0
        {
            return Err(WindowError::new(
                "unable to set XDG window manager listener - listener already set",
            ));
        };

        Ok(())
    }
}
