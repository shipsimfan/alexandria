use crate::gpu::VulkanSwapchainPresentMode;
use vulkan::khr_surface::VkPresentModeKhr;

impl VulkanSwapchainPresentMode {
    /// Convert a Vulkan present mode into a [`SwapchainPresentMode`]
    pub(in crate::gpu) fn from_vk(vk: VkPresentModeKhr) -> Option<VulkanSwapchainPresentMode> {
        match vk {
            VkPresentModeKhr::ImmediateModeKhr => Some(VulkanSwapchainPresentMode::Immediate),
            VkPresentModeKhr::MailboxKhr => Some(VulkanSwapchainPresentMode::Mailbox),
            VkPresentModeKhr::FIFOKhr => Some(VulkanSwapchainPresentMode::Fifo),
            VkPresentModeKhr::FIFORelaxedKhr => Some(VulkanSwapchainPresentMode::FifoRelaxed),
            _ => None,
        }
    }
}
