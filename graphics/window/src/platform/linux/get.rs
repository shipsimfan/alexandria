use crate::{Window, WindowEvents, WindowState, WindowWakeHandle};

impl<Callbacks: WindowEvents> Window<Callbacks> {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        match self {
            Window::Wayland(wayland) => wayland.state(),
            Window::X11(_) => todo!(),
        }
    }

    /// Get a reference that can be used to wake this thread if blocking for messages
    pub fn wake_handle(&self) -> WindowWakeHandle {
        match self {
            Window::Wayland(wayland) => wayland.wake_handle(),
            Window::X11(_) => todo!(),
        }
    }

    /// Get a reference to the window event callback item
    pub fn callbacks(&self) -> &Callbacks {
        match self {
            Window::Wayland(wayland) => wayland.callbacks(),
            Window::X11(_) => todo!(),
        }
    }

    /// Get a mutable reference to the window event callback item
    pub fn callbacks_mut(&mut self) -> &mut Callbacks {
        match self {
            Window::Wayland(wayland) => wayland.callbacks_mut(),
            Window::X11(_) => todo!(),
        }
    }
}
