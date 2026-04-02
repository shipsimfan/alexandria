use crate::gpu::VulkanSwapchainPresentMode;
use vulkan::khr_surface::VkPresentModeKhr;

impl VulkanSwapchainPresentMode {
    /// Convert this present mode into its Vulkan counter-part
    pub(in crate::gpu::device::swapchain) fn into_vk(self) -> VkPresentModeKhr {
        match self {
            VulkanSwapchainPresentMode::Immediate => VkPresentModeKhr::ImmediateModeKhr,
            VulkanSwapchainPresentMode::Mailbox => VkPresentModeKhr::MailboxKhr,
            VulkanSwapchainPresentMode::Fifo => VkPresentModeKhr::FIFOKhr,
            VulkanSwapchainPresentMode::FifoRelaxed => VkPresentModeKhr::FIFORelaxedKhr,
        }
    }
}
