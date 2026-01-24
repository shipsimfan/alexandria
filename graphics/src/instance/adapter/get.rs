use crate::{
    GraphicsAdapter, GraphicsAdapterKind, GraphicsInstance, GraphicsQueueFamilyInfo,
    GraphicsVersion,
};
use alexandria_util::{MemorySize, UUID};
use vulkan::VkPhysicalDevice;

impl<'instance> GraphicsAdapter<'instance> {
    /// Get the Vulkan version supported by this adapter
    pub fn api_version(&self) -> GraphicsVersion {
        self.api_version
    }

    /// Get the version of the driver this adapter uses
    pub fn driver_version(&self) -> GraphicsVersion {
        self.driver_version
    }

    /// Get the kind of graphics adapter this is
    pub fn kind(&self) -> GraphicsAdapterKind {
        self.kind
    }

    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// A UUID identifying the device on the system
    pub fn uuid(&self) -> &UUID {
        &self.uuid
    }

    /// Get the amount of video RAM the adapter has access to
    pub fn vram(&self) -> MemorySize {
        self.vram
    }

    /// Get the queue families on the adapter
    pub fn queue_families(&self) -> &[GraphicsQueueFamilyInfo] {
        &self.queue_families
    }

    /// Get the graphics instance this adapter came from
    pub(crate) fn instance(&self) -> &'instance GraphicsInstance {
        self.instance
    }

    /// Get access to the physical device handle
    pub(crate) fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }
}
