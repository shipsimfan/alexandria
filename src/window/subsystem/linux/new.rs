use crate::{EventQueue, Result, window::subsystem::WindowSubsystemInner};
use std::marker::PhantomData;

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Create a new [`WindowSubsystemInner`]
    pub(in crate::window::subsystem) fn new(
        event_queue: EventQueue<UserEvent>,
    ) -> Result<WindowSubsystemInner<UserEvent>> {
        // TODO: Actually implement
        Ok(WindowSubsystemInner::Wayland(PhantomData))
    }
}
