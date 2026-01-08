use crate::{
    GraphicsDebugMessageSeverity, GraphicsDebugMessenger, GraphicsDebugMessengerCallback,
    GraphicsInstance, Result,
};

impl GraphicsInstance {
    /// Create a new [`GraphicsDebugMessenger`]
    pub fn create_debug_messenger<C: GraphicsDebugMessengerCallback>(
        &self,
        min_severity: GraphicsDebugMessageSeverity,
        callback: C,
    ) -> Result<GraphicsDebugMessenger<C>> {
        self.inner
            .clone()
            .create_debug_messenger(min_severity, callback)
    }
}
