use crate::render_context::RenderContext;
use alexandria::gpu::VulkanDevice;
use std::ops::Deref;

impl Deref for RenderContext {
    type Target = VulkanDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
