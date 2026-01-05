use crate::platform::linux::wayland::WlRegistryRef;
use std::ffi::CStr;

mod add;
mod trampolines;

/// An item which can be used at the callback to registry events
pub(in crate::platform::linux::wayland) trait WlRegistryListener:
    Sized
{
    /// Called when a new global is registered
    fn global(&mut self, registry: WlRegistryRef, name: u32, interface: &CStr, version: u32);
}
