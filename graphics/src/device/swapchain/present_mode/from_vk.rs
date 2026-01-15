use crate::SwapchainPresentMode;
use vulkan::khr_surface::VkPresentModeKhr;

impl SwapchainPresentMode {
    /// Convert a Vulkan present mode into a [`SwapchainPresentMode`]
    pub(crate) fn from_vk(vk: VkPresentModeKhr) -> Option<SwapchainPresentMode> {
        match vk {
            VkPresentModeKhr::ImmediateModeKhr => Some(SwapchainPresentMode::Immediate),
            VkPresentModeKhr::MailboxKhr => Some(SwapchainPresentMode::Mailbox),
            VkPresentModeKhr::FIFOKhr => Some(SwapchainPresentMode::Fifo),
            VkPresentModeKhr::FIFORelaxedKhr => Some(SwapchainPresentMode::FifoRelaxed),
            _ => None,
        }
    }
}
