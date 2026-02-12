use std::rc::Rc;

#[cfg(target_os = "windows")]
use windows::WindowSubsystemInner;

#[cfg(target_os = "windows")]
mod windows;

mod new;
mod pump_events;
mod wait_for_event;

/// Allows interaction with the platform windowing system
#[derive(Clone)]
pub struct WindowSubsystem {
    /// The platform specific implementation of the subsystem
    inner: Rc<WindowSubsystemInner>,
}
