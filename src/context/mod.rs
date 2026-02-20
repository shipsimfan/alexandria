use crate::define_local_handle;
use inner::AlexandriaContextInner;

mod builder;
mod inner;

mod get;
mod new;

pub use builder::AlexandriaContextBuilder;

define_local_handle!(
    /// The main entry point for interacting with Alexandria
    pub AlexandriaContext<UserEvent: 'static + Send> -> AlexandriaContextInner<UserEvent>
);
