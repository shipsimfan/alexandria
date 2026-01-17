//! The Wayland implementation of the window system

use crate::{WindowEvents, WindowWakeHandleInner};
use event_handler::WaylandEventHandler;
use globals::WaylandGlobals;
use library::WaylandLibrary;
use std::{rc::Rc, sync::Arc};

mod event_handler;
mod globals;
mod library;
mod objects;

mod get;
mod new;
mod process_messages;
mod surface_creation_handle;

pub(in crate::platform::linux) use objects::*;

/// A window on a Wayland connection
pub struct WaylandWindow<Callbacks: WindowEvents> {
    /// The handle used to wake this thread if blocking for messages
    wake_handle: Arc<WindowWakeHandleInner>,

    /// The surface created by the XDG window manager
    toplevel_surface: XdgToplevel<WaylandEventHandler<Callbacks>>,

    /// The global Wayland registry
    #[allow(unused)]
    registry: WlRegistry<WaylandGlobals>,

    /// The main Wayland connection
    display: Rc<WlDisplay>,
}
