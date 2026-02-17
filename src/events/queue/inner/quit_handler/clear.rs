use crate::events::queue::inner::quit_handler::{QUIT_NOTIFY, os};

/// Clear the quit handler for this process
pub(in crate::events::queue::inner) fn clear_quit_handler() {
    os::clear_quit_handler();

    let mut quit_notify = QUIT_NOTIFY.lock().unwrap();
    *quit_notify = None;
    drop(quit_notify);
}
