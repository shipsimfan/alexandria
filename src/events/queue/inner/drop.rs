use crate::events::queue::inner::{EventQueueInner, clear_quit_handler};

impl<UserEvent: Send> Drop for EventQueueInner<UserEvent> {
    fn drop(&mut self) {
        clear_quit_handler();
    }
}
