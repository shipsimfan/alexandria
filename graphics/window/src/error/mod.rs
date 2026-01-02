use crate::platform;
use std::borrow::Cow;

mod display;
mod error;
mod new;

/// A result of a windowing call
pub type Result<T> = std::result::Result<T, WindowError>;

/// An error that can occur while running a value in the graphics API
#[derive(Debug)]
pub struct WindowError {
    /// The message describing the circumstances of the error, or the error itself
    message: Cow<'static, str>,

    /// The error from the windowing system, if it originated from there
    os: Option<platform::OsError>,
}
