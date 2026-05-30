mod wayland;
mod window_surface_creation_handle;

mod get;
mod set;
mod set_id;

pub(in crate::window) use wayland::WaylandWindow;

pub(crate) use window_surface_creation_handle::WindowSurfaceCreationHandle;

/// The Linux-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) enum WindowInner<UserEvent: 'static + Send> {
    /// The Wayland implementation of a Window
    Wayland(WaylandWindow<UserEvent>),

    /// The X11 implementation of a window
    X11,
}
