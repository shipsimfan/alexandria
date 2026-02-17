use crate::{EventQueue, Result, window::subsystem::WindowSubsystemInner};
use std::ptr::null_mut;
use win32::{DispatchMessage, MSG, PM_REMOVE, PeekMessage, TranslateMessage};

impl WindowSubsystemInner {
    /// Pumps events from input devices and the window system onto the event queue
    pub(in crate::window::subsystem) fn pump_events<UserEvent: Send>(
        &mut self,
        pump: &EventQueue<UserEvent>,
    ) -> Result<()> {
        // Pump the events from Win32
        let mut msg = MSG::default();
        while unsafe { PeekMessage(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessage(&msg) };
        }

        // Handle global events
        self.message_window
            .pump_events(pump, &mut self.displays, &mut self.dxgi_factory)?;

        Ok(())
    }
}
