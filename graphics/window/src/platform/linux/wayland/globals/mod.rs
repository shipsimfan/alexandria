use crate::{Result, platform::linux::wayland::WlCompositor};
use std::ffi::CStr;

mod get;
mod new;
mod registry_listener;

/// The bound Wayland globals
pub(in crate::platform::linux::wayland) struct WaylandGlobals {
    /// The result of running callbacks
    dispatch_result: Result<()>,

    /// A reference to the global compositor
    compositor: Option<WlCompositor>,

    /// The name of the `compositor` interface
    compositor_name: &'static CStr,
}
