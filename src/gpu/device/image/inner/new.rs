use crate::{
    Error, Result,
    gpu::{
        VulkanDevice, VulkanFormat, VulkanImageCreateFlags, VulkanImageLayout, VulkanImageTiling,
        VulkanImageType, VulkanImageUsageFlags, VulkanSampleCountFlag, VulkanSharingMode,
        device::image::VulkanImageInner,
    },
    math::Vector3u,
};
use std::ptr::null;
use vulkan::{VkImage, VkImageCreateInfo, try_vulkan};

impl VulkanImageInner {
    /// Create a new [`VulkanImageInner`]
    pub fn new(
        flags: VulkanImageCreateFlags,
        image_type: VulkanImageType,
        format: VulkanFormat,
        extent: Vector3u,
        mip_levels: u32,
        array_layers: u32,
        samples: VulkanSampleCountFlag,
        tiling: VulkanImageTiling,
        usage: VulkanImageUsageFlags,
        sharing_mode: VulkanSharingMode,
        queue_family_indices: &[u32],
        initial_layout: VulkanImageLayout,
        device: &VulkanDevice,
    ) -> Result<VulkanImageInner> {
        let create_info = VkImageCreateInfo {
            flags,
            image_type,
            format,
            extent: extent.into(),
            mip_levels,
            array_layers,
            samples,
            tiling,
            usage,
            sharing_mode,
            queue_family_index_count: queue_family_indices.len() as u32,
            queue_family_indices: queue_family_indices.as_ptr(),
            initial_layout,
            ..Default::default()
        };

        let mut handle = VkImage::null();
        try_vulkan!((device.functions().image.create_image)(
            device.handle(),
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|error| Error::new_with("unable to create an image", error))?;

        Ok(VulkanImageInner {
            handle,
            destroy: true,
            device: device.clone(),
        })
    }

    /// Create a new [`VulkanImageInner`] from a Vulkan image handle
    pub fn from_handle(handle: VkImage, device: VulkanDevice) -> VulkanImageInner {
        VulkanImageInner {
            handle,
            destroy: false,
            device,
        }
    }
}
