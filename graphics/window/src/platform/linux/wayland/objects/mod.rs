//! Wayland object wrappers

mod compositor;
mod display;
mod registry;

pub(in crate::platform::linux) use display::*;

pub(in crate::platform::linux::wayland) use compositor::*;
pub(in crate::platform::linux::wayland) use registry::*;
