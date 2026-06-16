use crate::gpu::{VulkanFormat, VulkanPipelineRenderingCreateInfo};

impl<'a> VulkanPipelineRenderingCreateInfo<'a> {
    /// Get the view mask for this pipeline rendering create info
    pub fn view_mask(&self) -> u32 {
        self.inner.view_mask
    }

    /// Get the number of color attachments for this pipeline rendering create info
    pub fn color_attachment_count(&self) -> u32 {
        self.inner.color_attachment_count
    }

    /// Get the color attachment formats for this pipeline rendering create info
    pub fn color_attachment_formats(&self) -> &[VulkanFormat] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.color_attachment_formats,
                self.inner.color_attachment_count as _,
            )
        }
    }

    /// Get the depth attachment format for this pipeline rendering create info
    pub fn depth_attachment_format(&self) -> VulkanFormat {
        self.inner.depth_attachment_format
    }

    /// Get the stencil attachment format for this pipeline rendering create info
    pub fn stencil_attachment_format(&self) -> VulkanFormat {
        self.inner.stencil_attachment_format
    }
}
