use std::borrow::Cow;

mod inner;

mod display;
mod error;
mod new;

pub use inner::ErrorInner;

/// A result of an Alexandria function
pub type Result<T> = std::result::Result<T, Error>;

/// An error that Alexandria can produce
#[derive(Debug)]
pub struct Error {
    /// The message describing the error
    pub message: Cow<'static, str>,

    /// The inner error giving more details
    pub inner: Option<ErrorInner>,
}
