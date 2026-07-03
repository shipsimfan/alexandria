mod wl_compositor;
mod wl_display;
mod wl_keyboard;
mod wl_registry;
mod wl_seat;
mod wl_surface;
mod xdg_decoration_manager;
mod xdg_output_manager;
mod xdg_surface;
mod xdg_top_level;
mod xdg_top_level_decoration;
mod xdg_wm_base;

pub(in crate::window) use wl_compositor::WlCompositor;
pub(in crate::window) use wl_display::WlDisplay;
pub(in crate::window) use wl_keyboard::{WlKeyboard, WlKeyboardListener};
pub(in crate::window) use wl_registry::{WaylandBind, WlRegistryRef};
pub(in crate::window) use wl_seat::{WlSeat, WlSeatListener, WlSeatRef};
pub(in crate::window) use wl_surface::WlSurface;
pub(in crate::window) use xdg_decoration_manager::XdgDecorationManager;
pub(in crate::window) use xdg_output_manager::XdgOutputManager;
pub(in crate::window) use xdg_surface::{XdgSurface, XdgSurfaceListener, XdgSurfaceRef};
pub(in crate::window) use xdg_top_level::{XdgTopLevel, XdgTopLevelListener};
pub(in crate::window) use xdg_top_level_decoration::{
    XdgTopLevelDecoration, XdgTopLevelDecorationListener,
};
pub(in crate::window) use xdg_wm_base::XdgWmBase;

pub(in crate::window::subsystem::linux::wayland) use wl_registry::{
    WlRegistry, WlRegistryListener,
};
