use crate::{Result, window::display::DisplayInner};
use std::marker::PhantomData;

impl DisplayInner {
    /// Create a new [`DisplayInner`]
    fn new() -> Result<DisplayInner> {
        Ok(DisplayInner { _priv: PhantomData })
    }
}
