use crate::gpu::{VulkanDynamicState, VulkanPipelineDynamicStateCreateInfo};

impl<'a> VulkanPipelineDynamicStateCreateInfo<'a> {
    /// Get the number of dynamic states in the pipeline
    pub fn dynamic_state_count(&self) -> usize {
        self.inner.dynamic_state_count as _
    }

    /// Get the dynamic states of the pipeline
    pub fn dynamic_states(&self) -> &'a [VulkanDynamicState] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.dynamic_states,
                self.inner.dynamic_state_count as _,
            )
        }
    }
}
