use crate::{
    Result,
    window::{WindowSubsystem, subsystem::WindowSubsystemInner},
};
use std::rc::Rc;

impl WindowSubsystem {
    /// Create a new [`WindowSubsystem`]
    pub(crate) fn new() -> Result<WindowSubsystem> {
        WindowSubsystemInner::new().map(|inner| WindowSubsystem {
            inner: Rc::new(inner),
        })
    }
}
