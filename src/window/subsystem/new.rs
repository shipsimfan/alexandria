use crate::{Result, window::WindowSubsystem};
use std::marker::PhantomData;

impl WindowSubsystem {
    /// Create a new [`WindowSubsystem`]
    pub(crate) fn new() -> Result<WindowSubsystem> {
        Ok(WindowSubsystem { _priv: PhantomData })
    }
}
