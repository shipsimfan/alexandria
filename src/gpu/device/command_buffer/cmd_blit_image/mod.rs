use crate::gpu::{VulkanCommandBuffer, VulkanFilter, VulkanImage, VulkanImageLayout};

mod image_blit;

pub use image_blit::*;

impl VulkanCommandBuffer {
    /// Blit an data from one image to another
    pub fn cmd_blit_image(
        &mut self,
        src_image: &VulkanImage,
        src_image_layout: VulkanImageLayout,
        dst_image: &VulkanImage,
        dst_image_layout: VulkanImageLayout,
        regions: &[VulkanImageBlit],
        filter: VulkanFilter,
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_blit_image)(
                self.handle,
                src_image.handle(),
                src_image_layout.into(),
                dst_image.handle(),
                dst_image_layout.into(),
                regions.len() as u32,
                regions.as_ptr().cast(),
                filter.into(),
            )
        };
    }
}
