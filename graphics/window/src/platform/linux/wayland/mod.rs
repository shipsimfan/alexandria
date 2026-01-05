//! The Wayland implementation of the window system

use globals::WaylandGlobals;
use library::WaylandLibrary;
use std::rc::Rc;

mod globals;
mod library;
mod objects;

mod new;
mod process_messages;
mod wait_for_message;

pub(in crate::platform::linux) use objects::*;

/// A window on a Wayland connection
pub(in crate::platform::linux) struct WaylandWindow {
    /// The main Wayland connection
    display: Rc<WlDisplay>,

    /// The global Wayland registry
    registry: WlRegistry<WaylandGlobals>,
}
