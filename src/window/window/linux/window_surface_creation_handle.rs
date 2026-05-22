/// The handle used to create a surface for a window on Linux
pub(crate) enum WindowSurfaceCreationHandle {
    /// The Wayland implementation of a surface creation handle
    Wayland,

    /// The X11 implementation of a surface creation handle
    X11,
}
