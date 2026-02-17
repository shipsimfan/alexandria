use crate::{
    Error, Notify, Result,
    events::queue::inner::quit_handler::{QUIT_NOTIFY, QUIT_SIGNALLED, os},
};
use std::sync::atomic::Ordering;

/// Set the quit handler for this process
pub(in crate::events::queue::inner) fn set_quit_handler(notify: Notify) -> Result<()> {
    QUIT_SIGNALLED.store(false, Ordering::SeqCst);

    let mut quit_notify = QUIT_NOTIFY.lock().unwrap();
    *quit_notify = Some(notify);
    drop(quit_notify);

    os::set_quit_handler().map_err(|os| Error::new_with("unable to set OS quit handler", os))
}
