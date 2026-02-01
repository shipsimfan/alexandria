use std::borrow::Cow;

mod as_error;
mod display;
mod from;

/// A source of an error containing more details
#[derive(Debug)]
pub enum ErrorInner {
    /// Some other error source
    Other(Cow<'static, str>),
}
