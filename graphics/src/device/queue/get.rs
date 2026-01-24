use crate::{GraphicsDevice, GraphicsQueue};

impl GraphicsQueue {
    /// Get the queue family this queue is from
    pub fn queue_family(&self) -> u32 {
        self.queue_family
    }

    /// Get the device this queue is from
    pub fn device(&self) -> &GraphicsDevice {
        &self.device
    }
}
