use crate::{instance, window};
use common::DebugMessage;

mod kind;
mod r#macro;

pub use kind::ErrorKind;

pub struct Error {
    kind: ErrorKind,
    error: win32::Win32Error,
    debug_messages: Vec<DebugMessage>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, error: win32::Win32Error) -> Self {
        Error {
            kind,
            error,
            debug_messages: Vec::new(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn error(&self) -> &win32::Win32Error {
        &self.error
    }

    pub fn debug_messages(&self) -> &[DebugMessage] {
        &self.debug_messages
    }

    pub(crate) fn get_instance_debug_messages(&mut self, debug: &mut instance::Debug) {
        while let Some(message) = debug.pop_message() {
            self.debug_messages.push(message);
        }
    }

    pub(crate) fn get_3d_debug_messages(&mut self, debug: &mut window::graphics_3d::Debug) {
        while let Some(message) = debug.pop_message() {
            self.debug_messages.push(message);
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.kind, self.error)
    }
}
