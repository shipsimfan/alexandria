use std::borrow::Cow;

mod display;
mod new;

/// A result of an Alexandria call
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while running Alexandria
#[derive(Debug)]
pub struct Error {
    /// The message giving context to the error
    message: Cow<'static, str>,

    /// The OS error, if it originated from the OS
    os: Option<win32::Error>,
}

impl std::error::Error for Error {}
