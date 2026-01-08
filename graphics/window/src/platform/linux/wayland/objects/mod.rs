//! Wayland object wrappers

mod compositor;
mod display;
mod registry;
mod wl_surface;
mod xdg_surface;
mod xdg_toplevel;
mod xdg_wm_base;

pub(in crate::platform::linux) use display::*;

pub(in crate::platform::linux::wayland) use compositor::*;
pub(in crate::platform::linux::wayland) use registry::*;
pub(in crate::platform::linux::wayland) use wl_surface::*;
pub(in crate::platform::linux::wayland) use xdg_surface::*;
pub(in crate::platform::linux::wayland) use xdg_toplevel::*;
pub(in crate::platform::linux::wayland) use xdg_wm_base::*;
