use crate::{
    PackedMap,
    window::{Win32Window, display::DisplayInner},
};
use message_only_wnd_proc::MessageOnlyWndProc;
use win32::{ComPtr, dxgi::IDXGIFactory};

mod message_only_wnd_proc;

mod displays;
mod new;
mod pump_events;
mod wait_for_event;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Winodws
pub(in crate::window) struct WindowSubsystemInner {
    /// The current set of displays
    displays: PackedMap<DisplayInner>,

    /// An invisible window to receive messages
    message_window: Win32Window<MessageOnlyWndProc>,

    /// The DXGI factory for enumerating displays
    dxgi_factory: ComPtr<IDXGIFactory>,
}
