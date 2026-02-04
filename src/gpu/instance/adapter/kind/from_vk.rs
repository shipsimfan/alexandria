use crate::gpu::VulkanAdapterKind;
use vulkan::VkPhysicalDeviceType;

impl VulkanAdapterKind {
    /// Convert a [`VkPhysicalDeviceType`] into a [`GraphicsAdapterKind`]
    pub(in crate::gpu::instance::adapter) fn from_vk(
        vk: VkPhysicalDeviceType,
    ) -> VulkanAdapterKind {
        match vk {
            VkPhysicalDeviceType::DiscreteGPU => VulkanAdapterKind::Discrete,
            VkPhysicalDeviceType::IntegratedGPU => VulkanAdapterKind::Integrated,
            VkPhysicalDeviceType::VirtualGPU => VulkanAdapterKind::Virtual,
            VkPhysicalDeviceType::CPU => VulkanAdapterKind::Cpu,
            _ => VulkanAdapterKind::Other,
        }
    }
}
