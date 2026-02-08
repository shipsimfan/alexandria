use crate::{AlexandriaContext, AlexandriaContextBuilder, EventPump, Result};

impl<UserEvent: Send> AlexandriaContextBuilder<UserEvent> {
    /// Create a new [`AlexandriaContext`] and [`EventPump`] with the provided options
    pub fn create(self) -> Result<(AlexandriaContext<UserEvent>, EventPump<UserEvent>)> {
        AlexandriaContext::new(self.gpu, self.window)
    }
}
