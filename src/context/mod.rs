use std::{cell::RefCell, marker::PhantomData};

mod builder;

mod drop;
mod new;

pub use builder::AlexandriaContextBuilder;

/// The main entry point for interacting with Alexandria
pub struct AlexandriaContext {
    /// A value to prevent this from being made externally
    _priv: PhantomData<()>,
}

thread_local! {
    /// A boolean to make sure only a single [`AlexandriaContext`] exists on a given thread
    static ALEXANDRIA_CONTEXT_ACTIVE: RefCell<bool> = RefCell::new(false);
}
