use crate::window::{
    XdgTopLevel,
    window::linux::wayland::{WaylandEventHandler, WindowHandle},
};
use std::ops::{Deref, DerefMut};

impl<UserEvent: 'static + Send> Deref for WindowHandle<UserEvent> {
    type Target = XdgTopLevel<WaylandEventHandler<UserEvent>>;

    fn deref(&self) -> &Self::Target {
        match self {
            WindowHandle::Decorated(window) => window.top_level(),
            WindowHandle::Undecorated(window) => window,
        }
    }
}

impl<UserEvent: 'static + Send> DerefMut for WindowHandle<UserEvent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            WindowHandle::Decorated(window) => window.top_level_mut(),
            WindowHandle::Undecorated(window) => window,
        }
    }
}
