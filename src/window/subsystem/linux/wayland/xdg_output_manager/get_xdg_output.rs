use crate::{
    Error,
    window::{WlOutput, XdgOutput, XdgOutputListener, XdgOutputManager},
};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_output::zxdg_output_manager_v1_get_xdg_output_dyn;

impl XdgOutputManager {
    /// Get an [`XdgOutput`] for the given [`WlOutput`] if possible
    pub(in crate::window) fn get_xdg_output<T: XdgOutputListener>(
        self: &Rc<Self>,
        output: WlOutput<T>,
    ) -> Result<XdgOutput<T>, (Error, WlOutput<T>)> {
        let handle = unsafe {
            zxdg_output_manager_v1_get_xdg_output_dyn(
                self.handle,
                output.handle(),
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };
        if handle == null_mut() {
            return Err((Error::new("unable to get an XdgOutput"), output));
        }

        XdgOutput::new(handle, output, self.clone())
    }
}
