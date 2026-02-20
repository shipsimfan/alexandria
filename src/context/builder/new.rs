use crate::{AlexandriaContext, AlexandriaContextBuilder};
use std::marker::PhantomData;

impl<UserEvent: 'static + Send> AlexandriaContext<UserEvent> {
    /// Create a new [`AlexandriaContextBuilder`]
    pub fn builder() -> AlexandriaContextBuilder<UserEvent> {
        AlexandriaContextBuilder::new()
    }
}

impl<UserEvent: 'static + Send> AlexandriaContextBuilder<UserEvent> {
    /// Create a new [`AlexandriaContextBuilder`]
    pub fn new() -> AlexandriaContextBuilder<UserEvent> {
        AlexandriaContextBuilder {
            gpu: false,
            window: false,
            _user_event: PhantomData,
        }
    }
}
