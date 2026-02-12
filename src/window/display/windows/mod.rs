use std::marker::PhantomData;

mod new;

/// The implementation of [`Display`](crate::window::Display)s for Winodws
pub(in crate::window::display) struct DisplayInner {
    /// A value to prevent this type from being made externally
    _priv: PhantomData<()>,
}
