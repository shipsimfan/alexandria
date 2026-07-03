use crate::gpu::{VulkanDynamicState, VulkanPipelineDynamicStateCreateInfo};
use std::marker::PhantomData;
use vulkan::VkPipelineDynamicStateCreateInfo;

impl<'a> VulkanPipelineDynamicStateCreateInfo<'a> {
    /// Create a new [`VulkanPipelineDynamicStateCreateInfo`]
    pub fn new(
        dynamic_states: &'a [VulkanDynamicState],
    ) -> VulkanPipelineDynamicStateCreateInfo<'a> {
        VulkanPipelineDynamicStateCreateInfo {
            inner: VkPipelineDynamicStateCreateInfo {
                dynamic_state_count: dynamic_states.len() as _,
                dynamic_states: dynamic_states.as_ptr() as _,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
