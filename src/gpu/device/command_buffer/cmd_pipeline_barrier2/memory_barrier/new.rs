use crate::gpu::{VulkanAccessFlags, VulkanMemoryBarrier, VulkanPipelineStageFlags};
use vulkan::VkMemoryBarrier2;

impl VulkanMemoryBarrier {
    /// Create a new [`VulkanMemoryBarrier`]
    pub fn new<
        P1: Into<VulkanPipelineStageFlags>,
        A1: Into<VulkanAccessFlags>,
        P2: Into<VulkanPipelineStageFlags>,
        A2: Into<VulkanAccessFlags>,
    >(
        src_stage_mask: P1,
        src_access_mask: A1,
        dst_stage_mask: P2,
        dst_access_mask: A2,
    ) -> VulkanMemoryBarrier {
        VulkanMemoryBarrier {
            inner: VkMemoryBarrier2 {
                src_stage_mask: src_stage_mask.into(),
                src_access_mask: src_access_mask.into(),
                dst_stage_mask: dst_stage_mask.into(),
                dst_access_mask: dst_access_mask.into(),
                ..Default::default()
            },
        }
    }
}
