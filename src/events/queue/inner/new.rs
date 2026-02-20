use crate::{
    Notify, Result,
    events::queue::{EventQueueInner, inner::set_quit_handler},
};
use std::{collections::VecDeque, sync::Mutex};

impl<UserEvent: 'static + Send> EventQueueInner<UserEvent> {
    /// Create a new [`EventQueueInner`]
    pub fn new() -> Result<EventQueueInner<UserEvent>> {
        let notify = Notify::new(false, false)?;

        set_quit_handler(notify.clone())?;

        Ok(EventQueueInner {
            queue: Mutex::new(VecDeque::new()),
            notify,
        })
    }
}
