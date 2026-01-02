use crate::{OsError, WindowError};
use std::borrow::Cow;

impl WindowError {
    /// Create a new [`WindowError`] from a windowing system error
    pub(crate) fn new_os<S: Into<Cow<'static, str>>>(message: S, os: OsError) -> WindowError {
        WindowError {
            message: message.into(),
            os: Some(os),
        }
    }
}
