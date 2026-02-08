mod kind;

mod new;

pub use kind::EventKind;

/// An event that can pushed into an [`EventQueue`](crate::EventQueue)
#[derive(Debug, Clone, PartialEq)]
pub struct Event<UserEvent: Send> {
    // TODO: Add Timestep
    /// The kind of event this is
    pub kind: EventKind<UserEvent>,
}
