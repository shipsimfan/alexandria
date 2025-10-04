mod display;
mod from;

/// A result of an Alexandria call
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while running Alexandria
#[derive(Debug)]
pub enum Error {
    /// There is no valid adapter in the system
    NoValidAdapter,

    /// The error is from the system
    System(win32::Error),
}

impl std::error::Error for Error {}
