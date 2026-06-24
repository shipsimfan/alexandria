use crate::gpu::{
    VulkanCompareOp, VulkanPipelineDepthStencilStateCreateFlags,
    VulkanPipelineDepthStencilStateCreateInfo, VulkanStencilOp,
};
use vulkan::{VK_FALSE, VK_TRUE, VkPipelineDepthStencilStateCreateInfo, VkStencilOpState};

impl VulkanPipelineDepthStencilStateCreateInfo {
    /// Create a new [`VulkanPipelineDepthStencilStateCreateInfo`]
    pub fn new<F: Into<VulkanPipelineDepthStencilStateCreateFlags>>(
        flags: F,
        depth_test_enable: bool,
        depth_write_enable: bool,
        depth_compare_op: VulkanCompareOp,
        depth_bounds_test_enable: bool,
        stencil_test_enable: bool,
        front_fail_op: VulkanStencilOp,
        front_pass_op: VulkanStencilOp,
        front_depth_fail_op: VulkanStencilOp,
        front_compare_op: VulkanCompareOp,
        front_compare_mask: u32,
        front_write_mask: u32,
        front_reference: u32,
        back_fail_op: VulkanStencilOp,
        back_pass_op: VulkanStencilOp,
        back_depth_fail_op: VulkanStencilOp,
        back_compare_op: VulkanCompareOp,
        back_compare_mask: u32,
        back_write_mask: u32,
        back_reference: u32,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> VulkanPipelineDepthStencilStateCreateInfo {
        VulkanPipelineDepthStencilStateCreateInfo {
            inner: VkPipelineDepthStencilStateCreateInfo {
                flags: flags.into(),
                depth_test_enable: if depth_test_enable { VK_TRUE } else { VK_FALSE },
                depth_write_enable: if depth_write_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                depth_compare_op,
                depth_bounds_test_enable: if depth_bounds_test_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                stencil_test_enable: if stencil_test_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                front: VkStencilOpState {
                    fail_op: front_fail_op,
                    pass_op: front_pass_op,
                    depth_fail_op: front_depth_fail_op,
                    compare_op: front_compare_op,
                    compare_mask: front_compare_mask,
                    write_mask: front_write_mask,
                    reference: front_reference,
                },
                back: VkStencilOpState {
                    fail_op: back_fail_op,
                    pass_op: back_pass_op,
                    depth_fail_op: back_depth_fail_op,
                    compare_op: back_compare_op,
                    compare_mask: back_compare_mask,
                    write_mask: back_write_mask,
                    reference: back_reference,
                },
                min_depth_bounds,
                max_depth_bounds,
                ..Default::default()
            },
        }
    }
}
