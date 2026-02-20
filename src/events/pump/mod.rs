use crate::{AlexandriaContext, EventQueue};

mod deref;
mod new;
mod poll;
mod pump;
mod wait;

/// A type that allows pumping events from the thread that initialized the
/// [`AlexandriaContext`](crate::AlexandriaContext)
pub struct EventPump<UserEvent: 'static + Send> {
    /// The queue to pump events onto
    queue: EventQueue<UserEvent>,

    /// The context this event pump is for
    context: AlexandriaContext<UserEvent>,
}
