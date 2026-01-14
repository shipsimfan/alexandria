//! The Wayland implementation of the window system

use crate::{WindowEvents, WindowState, WindowWakeHandleInner};
use globals::WaylandGlobals;
use library::WaylandLibrary;
use std::{rc::Rc, sync::Arc};

mod globals;
mod library;
mod objects;
mod state;

mod get;
mod new;
mod process_messages;

pub(in crate::platform::linux) use objects::*;

/// A window on a Wayland connection
pub(in crate::platform::linux) struct WaylandWindow<Callbacks: WindowEvents> {
    /// The handle used to wake this thread if blocking for messages
    wake_handle: Arc<WindowWakeHandleInner>,

    /// The surface created by the XDG window manager
    toplevel_surface: XdgToplevel<WindowState>,

    /// The global Wayland registry
    registry: WlRegistry<WaylandGlobals>,

    /// The main Wayland connection
    display: Rc<WlDisplay>,

    /// Callbacks for window events
    callbacks: Callbacks,
}
