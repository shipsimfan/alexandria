//! Wayland object wrappers

mod display;
mod registry;

pub(in crate::platform::linux) use display::*;

pub(in crate::platform::linux::wayland) use registry::*;
