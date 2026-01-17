use crate::{WindowEvents, WindowState, WindowWakeHandle, platform::linux::WaylandWindow};

impl<Callbacks: WindowEvents> WaylandWindow<Callbacks> {
    /// Get the current state of the window
    pub(in crate::platform::linux) fn state(&self) -> &WindowState {
        self.toplevel_surface.data().state()
    }

    /// Get a reference that can be used to wake this thread if blocking for messages
    pub(in crate::platform::linux) fn wake_handle(&self) -> WindowWakeHandle {
        WindowWakeHandle::new(self.wake_handle.clone())
    }

    /// Get a reference to the window event callback item
    pub(in crate::platform::linux) fn callbacks(&self) -> &Callbacks {
        self.toplevel_surface.data().callbacks()
    }

    /// Get a mutable reference to the window event callback item
    pub(in crate::platform::linux) fn callbacks_mut(&mut self) -> &mut Callbacks {
        self.toplevel_surface.data_mut().callbacks_mut()
    }
}
