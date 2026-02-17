use crate::{EventKind, EventQueue, Result, events::queue::inner::quit_handler::QUIT_SIGNALLED};
use std::sync::atomic::Ordering;

/// Check if a quit event has been signalled by the operating system
pub(in crate::events) fn pump_quit_event<UserEvent: Send>(
    event_pump: &EventQueue<UserEvent>,
) -> Result<()> {
    let quit_signalled = QUIT_SIGNALLED.swap(false, Ordering::Acquire);
    if quit_signalled {
        event_pump.push(EventKind::Quit)?;
    }
    Ok(())
}
