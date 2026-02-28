use std::{cell::RefCell, rc::Rc};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod clone;
mod create_window;
mod destroy_window;
mod get_displays;
mod get_windows;
mod new;
mod pump_events;
mod wait_for_event;

#[cfg(target_os = "linux")]
pub(in crate::window) use linux::{
    WaylandBind, WaylandFunctions, WaylandLibrary, WindowSubsystemInner, WlDisplay, WlRegistryRef,
    XdgOutputManager,
};
#[cfg(target_os = "windows")]
pub(in crate::window) use windows::WindowSubsystemInner;

/// Allows interaction with the platform windowing system
pub struct WindowSubsystem<UserEvent: 'static + Send> {
    /// The platform specific implementation of the subsystem
    inner: Rc<RefCell<WindowSubsystemInner<UserEvent>>>,
}
