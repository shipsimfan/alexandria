use crate::{
    MemorySize, Uuid,
    gpu::{VulkanAdapter, VulkanAdapterKind, VulkanInstance, VulkanQueueFamilyInfo, VulkanVersion},
};
use vulkan::VkPhysicalDevice;

impl<'instance> VulkanAdapter<'instance> {
    /// Get the Vulkan version supported by this adapter
    pub fn api_version(&self) -> VulkanVersion {
        self.api_version
    }

    /// Get the version of the driver this adapter uses
    pub fn driver_version(&self) -> VulkanVersion {
        self.driver_version
    }

    /// Get the kind of graphics adapter this is
    pub fn kind(&self) -> VulkanAdapterKind {
        self.kind
    }

    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// A UUID identifying the device on the system
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Get the amount of video RAM the adapter has access to
    pub fn vram(&self) -> MemorySize {
        self.vram
    }

    /// Get the queue families on the adapter
    pub fn queue_families(&self) -> &[VulkanQueueFamilyInfo] {
        &self.queue_families
    }

    /// Get the graphics instance this adapter came from
    pub(crate) fn instance(&self) -> &'instance VulkanInstance {
        self.instance
    }

    /// Get access to the physical device handle
    pub(crate) fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }
}
