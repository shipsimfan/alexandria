use crate::{EventIter, EventQueue};

impl<UserEvent: Send> EventQueue<UserEvent> {
    /// Get an iterator over the events without pumping new ones
    pub fn iter<'a>(&'a self) -> EventIter<'a, UserEvent> {
        EventIter { queue: self }
    }
}
