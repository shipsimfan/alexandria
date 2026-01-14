use crate::{Window, WindowEvents, WindowState, WindowWakeHandle, platform::linux::WindowKind};

impl<Callbacks: WindowEvents> Window<Callbacks> {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        match &self.kind {
            WindowKind::Wayland(wayland) => wayland.state(),
            WindowKind::X11(_) => todo!(),
        }
    }

    /// Get a reference that can be used to wake this thread if blocking for messages
    pub fn wake_handle(&self) -> WindowWakeHandle {
        match &self.kind {
            WindowKind::Wayland(wayland) => wayland.wake_handle(),
            WindowKind::X11(_) => todo!(),
        }
    }

    /// Get a reference to the window event callback item
    pub fn callbacks(&self) -> &Callbacks {
        match &self.kind {
            WindowKind::Wayland(wayland) => wayland.callbacks(),
            WindowKind::X11(_) => todo!(),
        }
    }

    /// Get a mutable reference to the window event callback item
    pub fn callbacks_mut(&mut self) -> &mut Callbacks {
        match &mut self.kind {
            WindowKind::Wayland(wayland) => wayland.callbacks_mut(),
            WindowKind::X11(_) => todo!(),
        }
    }
}
