use crate::events::queue::inner::quit_handler::{QUIT_NOTIFY, QUIT_SIGNALLED};
use std::sync::atomic::Ordering;

/// Signal a quit event
pub(in crate::events::queue::inner::quit_handler) fn quit_handler() {
    QUIT_SIGNALLED.store(true, Ordering::Release);
    if let Ok(quit_notify) = QUIT_NOTIFY.try_lock() {
        if let Some(quit_notify) = quit_notify.as_ref() {
            quit_notify.notify().ok();
        }
    }
}
