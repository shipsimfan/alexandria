use crate::gpu::{VulkanFormat, VulkanPipelineRenderingCreateInfo};

impl<'a> VulkanPipelineRenderingCreateInfo<'a> {
    /// Set the view mask for this pipeline rendering create info
    pub fn set_view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }

    /// Set the color attachment formats for this pipeline rendering create info
    pub fn set_color_attachment_formats(
        mut self,
        color_attachment_formats: &'a [VulkanFormat],
    ) -> Self {
        self.inner.color_attachment_count = color_attachment_formats.len() as _;
        self.inner.color_attachment_formats = color_attachment_formats.as_ptr();
        self
    }

    /// Set the depth attachment format for this pipeline rendering create info
    pub fn set_depth_attachment_format(mut self, depth_attachment_format: VulkanFormat) -> Self {
        self.inner.depth_attachment_format = depth_attachment_format;
        self
    }

    /// Set the stencil attachment format for this pipeline rendering create info
    pub fn set_stencil_attachment_format(
        mut self,
        stencil_attachment_format: VulkanFormat,
    ) -> Self {
        self.inner.stencil_attachment_format = stencil_attachment_format;
        self
    }
}
