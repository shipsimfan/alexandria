use crate::{
    GraphicsDebugMessageSeverity, GraphicsDebugMessenger, GraphicsDebugMessengerCallback, Result,
    instance::GraphicsInstanceInner,
};
use std::sync::Arc;

impl GraphicsInstanceInner {
    /// Create a new [`GraphicsDebugMessenger`]
    pub fn create_debug_messenger<C: GraphicsDebugMessengerCallback>(
        self: Arc<Self>,
        min_severity: GraphicsDebugMessageSeverity,
        callback: C,
    ) -> Result<GraphicsDebugMessenger<C>> {
        GraphicsDebugMessenger::new(self, min_severity, callback)
    }
}
