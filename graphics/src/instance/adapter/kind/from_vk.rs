use crate::GraphicsAdapterKind;
use vulkan::VkPhysicalDeviceType;

impl GraphicsAdapterKind {
    /// Convert a [`VkPhysicalDeviceType`] into a [`GraphicsAdapterKind`]
    pub(in crate::instance::adapter) fn from_vk(vk: VkPhysicalDeviceType) -> GraphicsAdapterKind {
        match vk {
            VkPhysicalDeviceType::DiscreteGPU => GraphicsAdapterKind::Discrete,
            VkPhysicalDeviceType::IntegratedGPU => GraphicsAdapterKind::Integrated,
            VkPhysicalDeviceType::VirtualGPU => GraphicsAdapterKind::Virtual,
            VkPhysicalDeviceType::CPU => GraphicsAdapterKind::Cpu,
            _ => GraphicsAdapterKind::Other,
        }
    }
}
