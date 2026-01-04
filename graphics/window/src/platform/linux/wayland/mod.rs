//! The Wayland implementation of the window system

use library::WaylandLibrary;

mod library;
mod objects;

pub(in crate::platform::linux) use objects::*;

/// A window on a Wayland connection
pub(in crate::platform::linux) struct WaylandWindow {}
