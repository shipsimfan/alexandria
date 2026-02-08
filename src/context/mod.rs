use crate::define_local_handle;

mod builder;
mod inner;

mod new;

pub use builder::AlexandriaContextBuilder;

pub(crate) use inner::AlexandriaContextInner;

define_local_handle!(
    /// The main entry point for interacting with Alexandria
    pub AlexandriaContext<UserEvent: Send> -> AlexandriaContextInner<UserEvent>
);
