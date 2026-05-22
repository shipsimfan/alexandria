use std::marker::PhantomData;

mod window_surface_creation_handle;

mod get;
mod set;

pub(crate) use window_surface_creation_handle::WindowSurfaceCreationHandle;

/// The Linux-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) enum WindowInner<UserEvent: 'static + Send> {
    /// The Wayland implementation of a Window
    Wayland(PhantomData<UserEvent>),

    /// The X11 implementation of a window
    X11,
}
