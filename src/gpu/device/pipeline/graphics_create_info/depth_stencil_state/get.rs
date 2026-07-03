use crate::gpu::{
    VulkanCompareOp, VulkanPipelineDepthStencilStateCreateFlags,
    VulkanPipelineDepthStencilStateCreateInfo, VulkanStencilOp,
};
use vulkan::VK_TRUE;

impl VulkanPipelineDepthStencilStateCreateInfo {
    /// Get the flags
    pub fn flags(&self) -> VulkanPipelineDepthStencilStateCreateFlags {
        self.inner.flags
    }

    /// Get if depth test is enabled
    pub fn depth_test_enable(&self) -> bool {
        self.inner.depth_test_enable == VK_TRUE
    }

    /// Get if depth write is enabled
    pub fn depth_write_enable(&self) -> bool {
        self.inner.depth_write_enable == VK_TRUE
    }

    /// Get the depth compare operation
    pub fn depth_compare_op(&self) -> VulkanCompareOp {
        self.inner.depth_compare_op
    }

    /// Get if depth bounds test is enabled
    pub fn depth_bounds_test_enable(&self) -> bool {
        self.inner.depth_bounds_test_enable == VK_TRUE
    }

    /// Get if stencil test is enabled
    pub fn stencil_test_enable(&self) -> bool {
        self.inner.stencil_test_enable == VK_TRUE
    }

    /// Get the front fail operation
    pub fn front_fail_op(&self) -> VulkanStencilOp {
        self.inner.front.fail_op
    }

    /// Get the front pass operation
    pub fn front_pass_op(&self) -> VulkanStencilOp {
        self.inner.front.pass_op
    }

    /// Get the front depth fail operation
    pub fn front_depth_fail_op(&self) -> VulkanStencilOp {
        self.inner.front.depth_fail_op
    }

    /// Get the front compare operation
    pub fn front_compare_op(&self) -> VulkanCompareOp {
        self.inner.front.compare_op
    }

    /// Get the front compare mask
    pub fn front_compare_mask(&self) -> u32 {
        self.inner.front.compare_mask
    }

    /// Get the front write mask
    pub fn front_write_mask(&self) -> u32 {
        self.inner.front.write_mask
    }

    /// Get the front reference
    pub fn front_reference(&self) -> u32 {
        self.inner.front.reference
    }

    /// Get the back fail operation
    pub fn back_fail_op(&self) -> VulkanStencilOp {
        self.inner.back.fail_op
    }

    /// Get the back pass operation
    pub fn back_pass_op(&self) -> VulkanStencilOp {
        self.inner.back.pass_op
    }

    /// Get the back depth fail operation
    pub fn back_depth_fail_op(&self) -> VulkanStencilOp {
        self.inner.back.depth_fail_op
    }

    /// Get the back compare operation
    pub fn back_compare_op(&self) -> VulkanCompareOp {
        self.inner.back.compare_op
    }

    /// Get the back compare mask
    pub fn back_compare_mask(&self) -> u32 {
        self.inner.back.compare_mask
    }

    /// Get the back write mask
    pub fn back_write_mask(&self) -> u32 {
        self.inner.back.write_mask
    }

    /// Get the back reference
    pub fn back_reference(&self) -> u32 {
        self.inner.back.reference
    }

    /// Get the minimum depth bounds
    pub fn min_depth_bounds(&self) -> f32 {
        self.inner.min_depth_bounds
    }

    /// Get the maximum depth bounds
    pub fn max_depth_bounds(&self) -> f32 {
        self.inner.max_depth_bounds
    }
}
