use std::marker::PhantomData;

mod create;
mod new;

/// A builder for an [`AlexandriaContext`](crate::AlexandriaContext)
pub struct AlexandriaContextBuilder {
    /// A value to prevent this from being made externally
    _priv: PhantomData<()>,
}
