use std::marker::PhantomData;

mod create;
mod new;
mod set;

/// A builder for an [`AlexandriaContext`](crate::AlexandriaContext)
pub struct AlexandriaContextBuilder<UserEvent: 'static + Send = ()> {
    /// Should the GPU subsystem be initialized?
    gpu: bool,

    /// Should the window subsystem be initialized? This implies `gpu` too
    window: bool,

    /// A marker for the user event type
    _user_event: PhantomData<UserEvent>,
}
