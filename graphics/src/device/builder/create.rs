use crate::{
    GraphicsDevice, GraphicsDeviceBuilder, GraphicsQueue, Result, device::GraphicsDeviceInner,
};
use std::sync::Arc;

impl<'adapter, 'instance, 'a> GraphicsDeviceBuilder<'adapter, 'instance, 'a> {
    /// Create a new [`GraphicsDevice`] with the provided settings
    pub fn create(&self) -> Result<(GraphicsDevice, Vec<GraphicsQueue>)> {
        let device = GraphicsDevice {
            inner: Arc::new(GraphicsDeviceInner::new(
                &self.extended_info,
                &self.queues,
                &self.extensions,
                self.adapter,
            )?),
        };

        let mut queues = Vec::new();
        for (queue_family_index, queue_family) in self.queues.iter().enumerate() {
            for queue in 0..queue_family.priorities.len() {
                queues.push(GraphicsQueue::new(
                    &device,
                    queue_family_index as _,
                    queue as _,
                ));
            }
        }

        Ok((device, queues))
    }
}
