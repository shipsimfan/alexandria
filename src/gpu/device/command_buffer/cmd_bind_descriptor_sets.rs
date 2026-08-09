use crate::gpu::{
    VulkanCommandBuffer, VulkanDescriptorSet, VulkanPipelineBindPoint, VulkanPipelineLayout,
};
use std::ptr::null;

impl VulkanCommandBuffer {
    /// Bind a descriptor set to the command buffer
    pub fn cmd_bind_descriptor_set(
        &mut self,
        pipeline_bind_point: VulkanPipelineBindPoint,
        layout: &VulkanPipelineLayout,
        index: u32,
        descriptor_set: &VulkanDescriptorSet,
    ) {
        unsafe {
            (self
                .device
                .functions()
                .command_buffer
                .cmd_bind_descriptor_sets)(
                self.handle,
                pipeline_bind_point,
                layout.handle(),
                index,
                1,
                &descriptor_set.handle(),
                0,
                null(),
            )
        }
    }
}
