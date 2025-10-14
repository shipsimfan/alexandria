use crate::Error;
use std::borrow::Cow;

impl Error {
    /// Create a new [`Error`]
    pub(crate) fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Error {
            message: message.into(),
            os: None,
        }
    }

    /// Create a new OS [`Error`]
    pub(crate) fn new_os<T: Into<Cow<'static, str>>>(message: T, os: win32::Error) -> Self {
        Error {
            message: message.into(),
            os: Some(os),
        }
    }
}
