use crate::events::queue::inner::quit_handler::quit_handler;
use win32::{BOOL, DWORD, SetConsoleCtrlHandler, TRUE, try_get_last_error};

unsafe extern "system" fn win32_quit_handler(_: DWORD) -> BOOL {
    quit_handler();
    TRUE
}

/// Sets the quit handler to run with the operating system
pub(in crate::events::queue::inner::quit_handler) fn set_quit_handler() -> win32::Result<()> {
    try_get_last_error!(SetConsoleCtrlHandler(Some(win32_quit_handler), TRUE)).map(|_| ())
}
