use crate::ErrorInner;
use std::borrow::Cow;

impl<T: Into<Cow<'static, str>>> From<T> for ErrorInner {
    fn from(error: T) -> Self {
        ErrorInner::Other(error.into())
    }
}
