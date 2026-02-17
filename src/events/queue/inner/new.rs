use crate::{
    Notify, Result,
    events::queue::{EventQueueInner, inner::set_quit_handler},
};
use std::{collections::VecDeque, sync::Mutex};

impl<UserEvent: Send> EventQueueInner<UserEvent> {
    /// Create a new [`EventQueueInner`]
    pub(in crate::events::queue) fn new() -> Result<EventQueueInner<UserEvent>> {
        let notify = Notify::new(false, false)?;

        set_quit_handler(notify.clone())?;

        Ok(EventQueueInner {
            queue: Mutex::new(VecDeque::new()),
            notify,
        })
    }
}
