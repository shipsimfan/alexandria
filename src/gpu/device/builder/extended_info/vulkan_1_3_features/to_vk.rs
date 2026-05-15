use crate::gpu::VulkanDeviceVulkan13Features;
use vulkan::{VK_FALSE, VK_TRUE, VkPhysicalDeviceVulkan13Features};

impl VulkanDeviceVulkan13Features {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::gpu::device::builder::extended_info) fn to_vk(
        &self,
    ) -> VkPhysicalDeviceVulkan13Features {
        VkPhysicalDeviceVulkan13Features {
            synchronization2: if self.synchorization2 {
                VK_TRUE
            } else {
                VK_FALSE
            },
            dynamic_rendering: if self.dynamic_rendering {
                VK_TRUE
            } else {
                VK_FALSE
            },
            ..Default::default()
        }
    }
}
