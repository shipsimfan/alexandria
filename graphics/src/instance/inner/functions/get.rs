use crate::instance::{
    GraphicsDebugMessengerFunctions, inner::functions::GraphicsInstanceFunctions,
};

impl GraphicsInstanceFunctions {
    /// Get the functions for debug messengers
    pub fn debug_messenger(&self) -> &GraphicsDebugMessengerFunctions {
        self.debug_messenger.as_ref().unwrap()
    }
}
