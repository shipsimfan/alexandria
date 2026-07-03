use crate::{
    gpu::{
        VulkanCompositeAlphaFlags, VulkanImageUsageFlags, VulkanSurfaceCapabilities,
        VulkanSurfaceTransformFlag, VulkanSurfaceTransformFlags,
    },
    math::Vector2u,
};

impl VulkanSurfaceCapabilities {
    /// Get the minimum number of images that the swapchain must support for the surface
    pub fn min_image_count(&self) -> u32 {
        self.inner.min_image_count
    }

    /// Get the maximum number of images that the swapchain must support for the surface
    pub fn max_image_count(&self) -> Option<u32> {
        if self.inner.max_image_count == 0 {
            None
        } else {
            Some(self.inner.max_image_count)
        }
    }

    /// Clamp the given image count to the range of supported image counts for the surface
    pub fn clamp_image_count(&self, image_count: u32) -> u32 {
        if let Some(max_image_count) = self.max_image_count() {
            image_count.clamp(self.min_image_count(), max_image_count)
        } else {
            image_count.max(self.min_image_count())
        }
    }

    /// Get the current extent of the surface
    pub fn current_extent(&self) -> Option<Vector2u> {
        if self.inner.current_extent.width == u32::MAX {
            None
        } else {
            Some(self.inner.current_extent.into())
        }
    }

    /// Get the minimum supported extent of the surface
    pub fn min_image_extent(&self) -> Vector2u {
        self.inner.min_image_extent.into()
    }

    /// Get the maximum supported extent of the surface
    pub fn max_image_extent(&self) -> Vector2u {
        self.inner.max_image_extent.into()
    }

    /// Clamp the given extent to the range of supported extents for the surface
    pub fn clamp_image_extent(&self, extent: Vector2u) -> Vector2u {
        extent.clamp_v(self.min_image_extent(), self.max_image_extent())
    }

    /// Get the maximum number of layers that the swapchain must support for the surface
    pub fn max_image_array_layers(&self) -> u32 {
        self.inner.max_image_array_layers
    }

    /// Get the supported transforms for the surface
    pub fn supported_transforms(&self) -> VulkanSurfaceTransformFlags {
        self.inner.supported_transforms
    }

    /// Get the current transform of the surface
    pub fn current_transform(&self) -> VulkanSurfaceTransformFlag {
        self.inner.current_transform
    }

    /// Get the supported composite alpha modes for the surface
    pub fn supported_composite_alpha(&self) -> VulkanCompositeAlphaFlags {
        self.inner.supported_composite_alpha
    }

    /// Get the supported usage flags for the surface
    pub fn supported_usage_flags(&self) -> VulkanImageUsageFlags {
        self.inner.supported_usage_flags
    }
}
