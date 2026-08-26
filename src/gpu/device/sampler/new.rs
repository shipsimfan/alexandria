use crate::{
    Error, Result,
    gpu::{
        VulkanBorderColor, VulkanCompareOp, VulkanDevice, VulkanFilter, VulkanSampler,
        VulkanSamplerAddressMode, VulkanSamplerCreateFlags, VulkanSamplerMipmapMode,
    },
};
use std::ptr::null;
use vulkan::{VK_FALSE, VK_TRUE, VkSampler, VkSamplerCreateInfo, try_vulkan};

impl VulkanSampler {
    /// Create a new [`VulkanSampler`]
    pub(in crate::gpu::device) fn new(
        flags: VulkanSamplerCreateFlags,
        mag_filter: VulkanFilter,
        min_filter: VulkanFilter,
        mipmap_mode: VulkanSamplerMipmapMode,
        address_mode_u: VulkanSamplerAddressMode,
        address_mode_v: VulkanSamplerAddressMode,
        address_mode_w: VulkanSamplerAddressMode,
        mip_lod_bias: f32,
        anisotropy_enable: bool,
        max_anisotropy: f32,
        compare_enable: bool,
        compare_op: VulkanCompareOp,
        min_lod: f32,
        max_lod: f32,
        border_color: VulkanBorderColor,
        unnormalized_coordinates: bool,
        device: &VulkanDevice,
    ) -> Result<VulkanSampler> {
        let create_info = VkSamplerCreateInfo {
            flags,
            mag_filter,
            min_filter,
            mipmap_mode,
            address_mode_u,
            address_mode_v,
            address_mode_w,
            mip_lod_bias,
            anisotropy_enable: if anisotropy_enable { VK_TRUE } else { VK_FALSE },
            max_anisotropy,
            compare_enable: if compare_enable { VK_TRUE } else { VK_FALSE },
            compare_op,
            min_lod,
            max_lod,
            border_color,
            unnormalized_coordinates: if unnormalized_coordinates {
                VK_TRUE
            } else {
                VK_FALSE
            },
            ..Default::default()
        };

        let mut handle = VkSampler::null();
        try_vulkan!((device.functions().sampler.create_sampler)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map_err(|error| Error::new_with("unable to create a sampler", error))?;

        Ok(VulkanSampler {
            handle,
            device: device.clone(),
        })
    }
}
