use crate::{
    GraphicsDebugMessenger, GraphicsDebugMessengerCallback, Result,
    instance::inner::GraphicsInstanceInner,
};

impl GraphicsInstanceInner {
    /// Create a new [`GraphicsDebugMessenger`]
    pub fn create_debug_messenger<C: GraphicsDebugMessengerCallback>(
        &self,
        callback: C,
    ) -> Result<GraphicsDebugMessenger<C>> {
        GraphicsDebugMessenger::new(callback)
    }
}
