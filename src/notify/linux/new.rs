use crate::notify::NotifyInner;
use linux::{
    Result,
    sys::eventfd::{EFD_NONBLOCK, eventfd},
    try_linux,
};

impl NotifyInner {
    /// Create a new [`NotifyInner`]
    pub fn new(auto_reset: bool, initial_state: bool) -> Result<NotifyInner> {
        let handle = try_linux!(eventfd(if initial_state { 1 } else { 0 }, EFD_NONBLOCK))?;

        Ok(NotifyInner { handle, auto_reset })
    }
}
