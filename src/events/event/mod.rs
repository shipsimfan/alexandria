use std::time::Instant;

mod kind;

mod new;

pub use kind::EventKind;

/// An event that can pushed into an [`EventQueue`](crate::EventQueue)
#[derive(Debug, Clone, PartialEq)]
pub struct Event<UserEvent: 'static + Send> {
    /// The time the event was pushed onto the queue
    pub time: Instant,

    /// The kind of event this is
    pub kind: EventKind<UserEvent>,
}
