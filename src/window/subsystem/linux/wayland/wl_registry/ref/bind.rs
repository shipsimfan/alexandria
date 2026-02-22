use crate::{
    Error, Result,
    window::subsystem::linux::{WlDisplay, wayland::WlRegistryRef},
};
use std::{ffi::CStr, ptr::null_mut, rc::Rc};
use wayland::{wl_interface, wl_registry_bind_dyn};

/// An item which is bindable from the Wayland registry
pub(in crate::window::subsystem::linux::wayland) trait WaylandBind {
    /// The handle type for this item
    type Handle;

    /// The interface describing the type
    const INTERFACE: &wl_interface;

    /// Create this item from a handle and source display
    fn from_handle(handle: *mut Self::Handle, display: Rc<WlDisplay>) -> Self;
}

impl<'a> WlRegistryRef<'a> {
    /// Bind the global at `name`
    pub fn bind<T: WaylandBind>(&mut self, name: u32, version: u32) -> Result<T> {
        let handle = unsafe {
            wl_registry_bind_dyn(
                self.handle,
                name,
                T::INTERFACE,
                version.min(T::INTERFACE.version as _),
                *self.display.library.f.proxy_marshal_flags,
            )
        };
        if handle == null_mut() {
            return Err(Error::new(format!(
                "unable to bind Wayland global \"{}\"",
                unsafe { CStr::from_ptr(T::INTERFACE.name) }.to_string_lossy()
            )));
        }

        Ok(T::from_handle(handle.cast(), self.display.clone()))
    }
}
