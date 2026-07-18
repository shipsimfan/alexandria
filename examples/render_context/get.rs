use crate::render_context::RenderContext;
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanCommandPool, VulkanQueue};

impl RenderContext {
    /// Get the memory properties for this context
    #[allow(unused)]
    pub fn memory_properties(&self) -> &VulkanAdapterMemoryProperties {
        &self.memory_properties
    }

    /// Get the command pool for this context
    #[allow(unused)]
    pub fn command_pool(&mut self) -> (&mut VulkanCommandPool, &mut VulkanQueue) {
        (&mut self.command_pool, &mut self.queue)
    }
}
