use crate::{OsError, WindowError};
use std::borrow::Cow;

impl WindowError {
    /// Create a new [`WindowError`] without a windowing system error
    #[cfg_attr(target_os = "windows", allow(unused))]
    pub(crate) fn new<S: Into<Cow<'static, str>>>(message: S) -> WindowError {
        WindowError {
            message: message.into(),
            os: None,
        }
    }

    /// Create a new [`WindowError`] from a windowing system error
    pub(crate) fn new_os<S: Into<Cow<'static, str>>, T: Into<OsError>>(
        message: S,
        os: T,
    ) -> WindowError {
        WindowError {
            message: message.into(),
            os: Some(os.into()),
        }
    }
}
