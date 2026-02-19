use std::{cell::RefCell, rc::Rc};

#[cfg(target_os = "windows")]
mod windows;

mod create_window;
mod displays;
mod new;
mod pump_events;
mod wait_for_event;

#[cfg(target_os = "windows")]
pub(in crate::window) use windows::WindowSubsystemInner;

/// Allows interaction with the platform windowing system
#[derive(Clone)]
pub struct WindowSubsystem {
    /// The platform specific implementation of the subsystem
    inner: Rc<RefCell<WindowSubsystemInner>>,
}
