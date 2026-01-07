use crate::{GraphicsDebugMessenger, GraphicsDebugMessengerCallback};
use std::ptr::null;

impl<C: GraphicsDebugMessengerCallback> Drop for GraphicsDebugMessenger<C> {
    fn drop(&mut self) {
        (self
            .instance
            .functions
            .debug_messenger()
            .destroy_debug_messenger)(self.instance.handle(), self.handle, null());
    }
}
