use crate::EventQueue;

mod iterator;
mod new;

/// An iterator which consumes events from a queue without pumping
pub struct EventIter<'a, UserEvent: 'static + Send> {
    /// The queue being read from
    queue: &'a EventQueue<UserEvent>,
}
