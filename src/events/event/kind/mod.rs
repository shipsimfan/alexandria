mod from;

/// A specific event kind that can pushed into an [`EventQueue`](crate::EventQueue)
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind<UserEvent: Send> {
    /// An event defined by the user of Alexandria
    User(UserEvent),
}
