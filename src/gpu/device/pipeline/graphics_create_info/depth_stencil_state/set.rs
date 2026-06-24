use crate::gpu::{
    VulkanCompareOp, VulkanPipelineDepthStencilStateCreateFlags,
    VulkanPipelineDepthStencilStateCreateInfo, VulkanStencilOp,
};
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanPipelineDepthStencilStateCreateInfo {
    /// Set the flags for the depth stencil state create info
    pub fn set_flags<F: Into<VulkanPipelineDepthStencilStateCreateFlags>>(
        mut self,
        flags: F,
    ) -> Self {
        self.inner.flags = flags.into();
        self
    }

    /// Enable the depth test for the depth stencil state create info
    pub fn enable_depth_test(mut self) -> Self {
        self.inner.depth_test_enable = VK_TRUE;
        self
    }

    /// Disable the depth test for the depth stencil state create info
    pub fn disable_depth_test(mut self) -> Self {
        self.inner.depth_test_enable = VK_FALSE;
        self
    }

    /// Enable the depth write for the depth stencil state create info
    pub fn enable_depth_write(mut self) -> Self {
        self.inner.depth_write_enable = VK_TRUE;
        self
    }

    /// Disable the depth write for the depth stencil state create info
    pub fn disable_depth_write(mut self) -> Self {
        self.inner.depth_write_enable = VK_FALSE;
        self
    }

    /// Set the depth compare operation for the depth stencil state create info
    pub fn set_depth_compare_op(mut self, depth_compare_op: VulkanCompareOp) -> Self {
        self.inner.depth_compare_op = depth_compare_op;
        self
    }

    /// Enable the depth bounds test for the depth stencil state create info
    pub fn enable_depth_bounds_test(mut self) -> Self {
        self.inner.depth_bounds_test_enable = VK_TRUE;
        self
    }

    /// Disable the depth bounds test for the depth stencil state create info
    pub fn disable_depth_bounds_test(mut self) -> Self {
        self.inner.depth_bounds_test_enable = VK_FALSE;
        self
    }

    /// Enable the stencil test for the depth stencil state create info
    pub fn enable_stencil_test(mut self) -> Self {
        self.inner.stencil_test_enable = VK_TRUE;
        self
    }

    /// Disable the stencil test for the depth stencil state create info
    pub fn disable_stencil_test(mut self) -> Self {
        self.inner.stencil_test_enable = VK_FALSE;
        self
    }

    /// Set the front fail operation for the depth stencil state create info
    pub fn set_front_fail_op(mut self, front_fail_op: VulkanStencilOp) -> Self {
        self.inner.front.fail_op = front_fail_op;
        self
    }

    /// Set the front pass operation for the depth stencil state create info
    pub fn set_front_pass_op(mut self, front_pass_op: VulkanStencilOp) -> Self {
        self.inner.front.pass_op = front_pass_op;
        self
    }

    /// Set the front depth fail operation for the depth stencil state create info
    pub fn set_front_depth_fail_op(mut self, front_depth_fail_op: VulkanStencilOp) -> Self {
        self.inner.front.depth_fail_op = front_depth_fail_op;
        self
    }

    /// Set the front compare operation for the depth stencil state create info
    pub fn set_front_compare_op(mut self, front_compare_op: VulkanCompareOp) -> Self {
        self.inner.front.compare_op = front_compare_op;
        self
    }

    /// Set the front compare mask for the depth stencil state create info
    pub fn set_front_compare_mask(mut self, front_compare_mask: u32) -> Self {
        self.inner.front.compare_mask = front_compare_mask;
        self
    }

    /// Set the front write mask for the depth stencil state create info  
    pub fn set_front_write_mask(mut self, front_write_mask: u32) -> Self {
        self.inner.front.write_mask = front_write_mask;
        self
    }

    /// Set the front reference for the depth stencil state create info
    pub fn set_front_reference(mut self, front_reference: u32) -> Self {
        self.inner.front.reference = front_reference;
        self
    }

    /// Set the back fail operation for the depth stencil state create info
    pub fn set_back_fail_op(mut self, back_fail_op: VulkanStencilOp) -> Self {
        self.inner.back.fail_op = back_fail_op;
        self
    }

    /// Set the back pass operation for the depth stencil state create info
    pub fn set_back_pass_op(mut self, back_pass_op: VulkanStencilOp) -> Self {
        self.inner.back.pass_op = back_pass_op;
        self
    }

    /// Set the back depth fail operation for the depth stencil state create info
    pub fn set_back_depth_fail_op(mut self, back_depth_fail_op: VulkanStencilOp) -> Self {
        self.inner.back.depth_fail_op = back_depth_fail_op;
        self
    }

    /// Set the back compare operation for the depth stencil state create info
    pub fn set_back_compare_op(mut self, back_compare_op: VulkanCompareOp) -> Self {
        self.inner.back.compare_op = back_compare_op;
        self
    }

    /// Set the back compare mask for the depth stencil state create info
    pub fn set_back_compare_mask(mut self, back_compare_mask: u32) -> Self {
        self.inner.back.compare_mask = back_compare_mask;
        self
    }

    /// Set the back write mask for the depth stencil state create info
    pub fn set_back_write_mask(mut self, back_write_mask: u32) -> Self {
        self.inner.back.write_mask = back_write_mask;
        self
    }

    /// Set the back reference for the depth stencil state create info
    pub fn set_back_reference(mut self, back_reference: u32) -> Self {
        self.inner.back.reference = back_reference;
        self
    }

    /// Set the minimum depth bounds for the depth stencil state create info
    pub fn set_min_depth_bounds(mut self, min_depth_bounds: f32) -> Self {
        self.inner.min_depth_bounds = min_depth_bounds;
        self
    }

    /// Set the maximum depth bounds for the depth stencil state create info
    pub fn set_max_depth_bounds(mut self, max_depth_bounds: f32) -> Self {
        self.inner.max_depth_bounds = max_depth_bounds;
        self
    }
}
