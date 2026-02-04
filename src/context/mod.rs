use crate::define_handle;

mod builder;
mod inner;

mod from_weak;
mod new;

pub use builder::AlexandriaContextBuilder;

pub(crate) use inner::AlexandriaContextInner;

define_handle!(
    /// The main entry point for interacting with Alexandria
    pub AlexandriaContext -> AlexandriaContextInner
);
