use crate::render_context::RenderContext;
use alexandria::gpu::VulkanAdapterMemoryProperties;

impl RenderContext {
    /// Get the memory properties for this context
    #[allow(unused)]
    pub fn memory_properties(&self) -> &VulkanAdapterMemoryProperties {
        &self.memory_properties
    }
}
