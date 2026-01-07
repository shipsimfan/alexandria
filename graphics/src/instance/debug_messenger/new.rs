use crate::{GraphicsDebugMessenger, GraphicsDebugMessengerCallback, Result};

impl<C: GraphicsDebugMessengerCallback> GraphicsDebugMessenger<C> {
    /// Create a new [`GraphicsDebugMessenger`]
    pub(in crate::instance) fn new(callback: C) -> Result<GraphicsDebugMessenger<C>> {
        todo!()
    }
}
