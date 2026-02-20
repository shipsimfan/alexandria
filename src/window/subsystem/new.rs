use crate::{
    EventQueue, Result,
    window::{WindowSubsystem, subsystem::WindowSubsystemInner},
};
use std::{cell::RefCell, rc::Rc};

impl<UserEvent: 'static + Send> WindowSubsystem<UserEvent> {
    /// Create a new [`WindowSubsystem`]
    pub(crate) fn new(event_queue: EventQueue<UserEvent>) -> Result<WindowSubsystem<UserEvent>> {
        WindowSubsystemInner::new(event_queue).map(|inner| WindowSubsystem {
            inner: Rc::new(RefCell::new(inner)),
        })
    }
}
