use linux::signal::{SIG_DFL, SIGINT, SIGTERM, sigaction, sigaction_handler, sigaction_t};
use std::ptr::null_mut;

/// Clear the quit handler with the operating system
pub(in crate::events::queue::inner::quit_handler) fn clear_quit_handler() {
    let action = sigaction_t {
        handler: sigaction_handler { handler: SIG_DFL },
        ..Default::default()
    };

    unsafe { sigaction(SIGTERM, &action, null_mut()) };
    unsafe { sigaction(SIGINT, &action, null_mut()) };
}
