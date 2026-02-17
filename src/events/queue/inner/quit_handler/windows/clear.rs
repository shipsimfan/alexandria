use win32::{FALSE, SetConsoleCtrlHandler};

/// Clear the quit handler with the operating system
pub(in crate::events::queue::inner::quit_handler) fn clear_quit_handler() {
    unsafe { SetConsoleCtrlHandler(None, FALSE) };
}
