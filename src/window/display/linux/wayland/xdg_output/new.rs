use crate::{
    Error,
    window::{WlOutput, XdgOutput, XdgOutputListener, XdgOutputManager},
};
use std::rc::Rc;
use wayland::xdg_output::{zxdg_output_v1, zxdg_output_v1_add_listener_dyn};

impl<T: XdgOutputListener> XdgOutput<T> {
    /// Create a new [`XdgOutput`]
    pub fn new(
        handle: *mut zxdg_output_v1,
        mut output: WlOutput<T>,
        manager: Rc<XdgOutputManager>,
    ) -> Result<XdgOutput<T>, (Error, WlOutput<T>)> {
        assert_ne!(
            handle,
            std::ptr::null_mut(),
            "XdgOutput handle cannot be null"
        );

        if unsafe {
            zxdg_output_v1_add_listener_dyn(
                handle,
                &XdgOutput::<T>::LISTENER,
                (output.data_mut() as *mut T).cast(),
                *manager.connection().library.f.proxy_add_listener,
            )
        } < 0
        {
            return Err((
                Error::new("unable to set XDG output listener - listener already set"),
                output,
            ));
        }

        Ok(XdgOutput {
            handle,
            wl_output: Some(output),
            manager,
        })
    }
}
