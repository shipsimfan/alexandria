use crate::Notify;
use handler::quit_handler;
use std::sync::{Mutex, atomic::AtomicBool};

#[cfg(target_os = "windows")]
use windows as os;

mod clear;
mod handler;
mod pump;
mod set;

#[cfg(target_os = "windows")]
mod windows;

pub(in crate::events) use pump::pump_quit_event;

pub(in crate::events::queue::inner) use clear::clear_quit_handler;
pub(in crate::events::queue::inner) use set::set_quit_handler;

/// Has the handler run and signalled a quit?
static QUIT_SIGNALLED: AtomicBool = AtomicBool::new(false);

/// The [`Notify`] to try and signal when the quit is signalled
static QUIT_NOTIFY: Mutex<Option<Notify>> = Mutex::new(None);
