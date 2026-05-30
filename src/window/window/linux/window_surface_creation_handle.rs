use wayland::{wl_display, wl_surface};

/// The handle used to create a surface for a window on Linux
pub(crate) enum WindowSurfaceCreationHandle {
    /// The Wayland implementation of a surface creation handle
    Wayland {
        /// The Wayland display for this window
        display: *mut wl_display,

        /// The Wayland surface for this window
        surface: *mut wl_surface,
    },

    /// The X11 implementation of a surface creation handle
    X11,
}
