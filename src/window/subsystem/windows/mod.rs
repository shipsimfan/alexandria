use crate::{
    EventQueue, PackedMap,
    window::{
        StandardWndProc, Win32Window, WindowClass, display::DisplayInner, window::WindowInner,
    },
};
use message_only_wnd_proc::MessageOnlyWndProc;
use std::rc::Rc;
use win32::{ComPtr, dxgi::IDXGIFactory};

mod message_only_wnd_proc;

mod create_window;
mod destroy_window;
mod get_displays;
mod get_windows;
mod new;
mod pump_events;
mod wait_for_event;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Windows
pub(in crate::window) struct WindowSubsystemInner<UserEvent: 'static + Send> {
    /// The current set of displays
    displays: PackedMap<DisplayInner<UserEvent>>,

    /// The current set of windows
    windows: PackedMap<WindowInner<UserEvent>>,

    /// The event queue to push events to
    event_queue: EventQueue<UserEvent>,

    /// An invisible window to receive messages
    message_window: Win32Window<MessageOnlyWndProc>,

    /// The window class used by standard windows
    standard_window_class: Rc<WindowClass<StandardWndProc<UserEvent>>>,

    /// The DXGI factory for enumerating displays
    dxgi_factory: ComPtr<IDXGIFactory>,
}
