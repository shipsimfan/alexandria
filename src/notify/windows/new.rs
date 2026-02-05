use crate::notify::NotifyInner;
use std::ptr::{null, null_mut};
use win32::{CreateEvent, FALSE, TRUE, try_get_last_error};

impl NotifyInner {
    /// Create a new [`NotifyInner`]
    pub fn new(auto_reset: bool, initial_state: bool) -> win32::Result<NotifyInner> {
        try_get_last_error!(CreateEvent(
            null_mut(),
            if auto_reset { TRUE } else { FALSE },
            if initial_state { TRUE } else { FALSE },
            null()
        ))
        .map(|handle| NotifyInner { handle })
    }
}
