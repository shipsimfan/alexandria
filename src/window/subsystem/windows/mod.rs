use crate::{PackedMap, window::display::DisplayInner};
use win32::{ComPtr, dxgi::IDXGIFactory};

mod displays;
mod new;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Winodws
pub(in crate::window) struct WindowSubsystemInner {
    /// The current set of displays
    displays: PackedMap<DisplayInner>,

    /// The DXGI factory for enumerating displays
    dxgi_factory: ComPtr<IDXGIFactory>,
}
