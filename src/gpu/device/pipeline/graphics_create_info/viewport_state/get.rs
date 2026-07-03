use crate::{
    gpu::{VulkanPipelineViewportStateCreateInfo, VulkanViewport},
    math::Recti,
};

impl<'a> VulkanPipelineViewportStateCreateInfo<'a> {
    /// Get the number of viewports
    pub fn viewport_count(&self) -> usize {
        self.inner.viewport_count as _
    }

    /// Get the viewports
    pub fn viewports(&self) -> &'a [VulkanViewport] {
        if self.inner.viewports.is_null() {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.inner.viewports.cast(),
                    self.inner.viewport_count as _,
                )
            }
        }
    }

    /// Get the number of scissors
    pub fn scissor_count(&self) -> usize {
        self.inner.scissor_count as _
    }

    /// Get the scissors
    pub fn scissors(&self) -> &'a [Recti] {
        if self.inner.scissors.is_null() {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.inner.scissors.cast(),
                    self.inner.scissor_count as _,
                )
            }
        }
    }
}
