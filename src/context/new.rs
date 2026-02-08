use crate::{AlexandriaContext, EventPump, Result, context::AlexandriaContextInner};

impl<UserEvent: Send> AlexandriaContext<UserEvent> {
    /// Create a new [`AlexandriaContext`] and [`EventPump`]
    pub(in crate::context) fn new(
        gpu: bool,
        window: bool,
    ) -> Result<(AlexandriaContext<UserEvent>, EventPump<UserEvent>)> {
        let context =
            AlexandriaContextInner::new(gpu, window).map(AlexandriaContext::from_inner)?;
        let event_pump = unsafe { EventPump::new(context.clone()) };
        Ok((context, event_pump))
    }
}
