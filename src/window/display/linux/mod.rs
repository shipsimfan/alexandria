mod get;

/// The implementation of [`Display`](crate::window::Display)s for Linux
pub(in crate::window) enum DisplayInner {
    /// The Wayland implementation of a display
    Wayland,

    /// The X11 implementation of a display
    X11,
}
