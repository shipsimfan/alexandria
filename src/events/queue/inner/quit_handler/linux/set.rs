use crate::events::queue::inner::quit_handler::quit_handler;
use linux::{
    signal::{SIGINT, SIGTERM, sigaction, sigaction_handler, sigaction_t, siginfo_t},
    try_linux,
};
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};

extern "C" fn linux_quit_handler(_: c_int, _: *mut siginfo_t, _: *mut c_void) {
    quit_handler();
}

/// Sets the quit handler to run with the operating system
pub(in crate::events::queue::inner::quit_handler) fn set_quit_handler() -> linux::Result<()> {
    let action = sigaction_t {
        handler: sigaction_handler {
            sigaction: Some(linux_quit_handler),
        },
        ..Default::default()
    };

    try_linux!(sigaction(SIGTERM, &action, null_mut()))?;
    try_linux!(sigaction(SIGINT, &action, null_mut()))?;
    Ok(())
}
