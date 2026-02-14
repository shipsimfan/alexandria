use crate::{
    Result,
    window::{WindowSubsystem, subsystem::WindowSubsystemInner},
};
use std::{cell::RefCell, rc::Rc};

impl WindowSubsystem {
    /// Create a new [`WindowSubsystem`]
    pub(crate) fn new() -> Result<WindowSubsystem> {
        WindowSubsystemInner::new().map(|inner| WindowSubsystem {
            inner: Rc::new(RefCell::new(inner)),
        })
    }
}
