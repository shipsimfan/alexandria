use crate::{Error, ErrorInner};
use std::borrow::Cow;

impl Error {
    /// Create a new [`Error`] without an inner error
    pub fn new<S: Into<Cow<'static, str>>>(message: S) -> Error {
        Error {
            message: message.into(),
            inner: None,
        }
    }

    /// Create a new [`Error`] with an inner error
    pub fn new_with<S: Into<Cow<'static, str>>, I: Into<ErrorInner>>(
        message: S,
        inner: I,
    ) -> Error {
        Error {
            message: message.into(),
            inner: Some(inner.into()),
        }
    }
}
