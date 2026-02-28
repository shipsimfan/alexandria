use crate::window::XdgOutput;
use wayland::xdg_output::zxdg_output_v1_destroy_dyn;

impl<T> Drop for XdgOutput<T> {
    fn drop(&mut self) {
        unsafe {
            zxdg_output_v1_destroy_dyn(
                self.handle,
                *self.manager.connection().library.f.proxy_marshal_flags,
                *self.manager.connection().library.f.proxy_get_version,
            )
        };
    }
}
