use crate::{
    Result,
    gpu::{
        VulkanBorderColor, VulkanCompareOp, VulkanDevice, VulkanFilter, VulkanSampler,
        VulkanSamplerAddressMode, VulkanSamplerCreateFlags, VulkanSamplerMipmapMode,
    },
};

impl VulkanDevice {
    /// Create a new [`VulkanSampler`]
    pub fn create_sampler<F: Into<VulkanSamplerCreateFlags>>(
        &self,
        flags: F,
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
    ) -> Result<VulkanSampler> {
        VulkanSampler::new(
            flags.into(),
            mag_filter,
            min_filter,
            mipmap_mode,
            address_mode_u,
            address_mode_v,
            address_mode_w,
            mip_lod_bias,
            anisotropy_enable,
            max_anisotropy,
            compare_enable,
            compare_op,
            min_lod,
            max_lod,
            border_color,
            unnormalized_coordinates,
            self,
        )
    }
}
