mod from;

/// A specific event kind that can pushed into an [`EventQueue`](crate::EventQueue)
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind<UserEvent: Send> {
    /// The process has been requested to be shutdown
    Quit,

    /// An event defined by the user of Alexandria
    User(UserEvent),
}
