use crate::window::{XdgTopLevel, XdgTopLevelDecoration, window::linux::wayland::WaylandEventHandler};

mod deref;

/// A handle to a Wayland window, which can either be decorated or undecorated
pub(in crate::window::window::linux::wayland) enum WindowHandle<UserEvent: 'static + Send> {
    /// The window is decorated with a title bar and borders by the window manager
    Decorated(XdgTopLevelDecoration<WaylandEventHandler<UserEvent>>),

    /// The window is undecorated as the window manager does not provide decorations for the window
    Undecorated(XdgTopLevel<WaylandEventHandler<UserEvent>>),
}
