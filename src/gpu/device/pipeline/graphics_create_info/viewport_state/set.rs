use crate::{
    gpu::{VulkanPipelineViewportStateCreateInfo, VulkanViewport},
    math::Recti,
};

impl<'a> VulkanPipelineViewportStateCreateInfo<'a> {
    /// Set the viewports to be used by the pipeline
    pub fn set_viewports(&mut self, viewports: &'a [VulkanViewport]) -> &mut Self {
        self.inner.viewport_count = viewports.len() as _;
        self.inner.viewports = viewports.as_ptr() as _;
        self
    }

    /// Set the scissors to be used by the pipeline
    pub fn set_scissors(&mut self, scissors: &'a [Recti]) -> &mut Self {
        self.inner.scissor_count = scissors.len() as _;
        self.inner.scissors = scissors.as_ptr() as _;
        self
    }
}
