use crate::{Result, window::subsystem::WindowSubsystemInner};
use std::marker::PhantomData;

impl WindowSubsystemInner {
    /// Create a new [`WindowSubsystemInner`]
    pub(crate) fn new() -> Result<WindowSubsystemInner> {
        Ok(WindowSubsystemInner { _priv: PhantomData })
    }
}
