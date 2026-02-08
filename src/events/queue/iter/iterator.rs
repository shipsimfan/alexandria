use crate::{Event, EventIter, Result};

impl<'a, UserEvent: Send> Iterator for EventIter<'a, UserEvent> {
    type Item = Result<Event<UserEvent>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop() {
            Ok(value) => value.map(Ok),
            Err(error) => Some(Err(error)),
        }
    }
}
