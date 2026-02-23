use std::marker::PhantomData;

mod get;

/// The Linux-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) enum WindowInner<UserEvent: 'static + Send> {
    /// The Wayland implementation of a Window
    Wayland(PhantomData<UserEvent>),

    /// The X11 implementation of a window
    X11,
}
