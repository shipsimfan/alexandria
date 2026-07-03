use crate::gpu::{VulkanDynamicState, VulkanPipelineDynamicStateCreateInfo};

impl<'a> VulkanPipelineDynamicStateCreateInfo<'a> {
    /// Set the dynamic states for this pipeline dynamic state create info
    pub fn set_dynamic_states(mut self, dynamic_states: &'a [VulkanDynamicState]) -> Self {
        self.inner.dynamic_state_count = dynamic_states.len() as _;
        self.inner.dynamic_states = dynamic_states.as_ptr() as _;
        self
    }
}
