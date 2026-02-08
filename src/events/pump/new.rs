use crate::{AlexandriaContext, EventPump};

impl<UserEvent: Send> EventPump<UserEvent> {
    /// Create a new [`EventPump`] for `queue`
    pub(crate) unsafe fn new(context: AlexandriaContext<UserEvent>) -> EventPump<UserEvent> {
        EventPump {
            queue: context.event_queue().clone(),
            context,
        }
    }
}
