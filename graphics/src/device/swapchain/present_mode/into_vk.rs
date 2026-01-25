use crate::SwapchainPresentMode;
use vulkan::khr_surface::VkPresentModeKhr;

impl SwapchainPresentMode {
    /// Convert this present mode into its Vulkan counter-part
    pub(in crate::device::swapchain) fn into_vk(self) -> VkPresentModeKhr {
        match self {
            SwapchainPresentMode::Immediate => VkPresentModeKhr::ImmediateModeKhr,
            SwapchainPresentMode::Mailbox => VkPresentModeKhr::MailboxKhr,
            SwapchainPresentMode::Fifo => VkPresentModeKhr::FIFOKhr,
            SwapchainPresentMode::FifoRelaxed => VkPresentModeKhr::FIFORelaxedKhr,
        }
    }
}
