use super::PhysicalDevice;
use crate::Instance;
use std::sync::Arc;
use vulkan::VkPhysicalDevice;

impl PhysicalDevice {
    /// Creates a new [`PhysicalDevice`]
    pub(crate) fn new(handle: VkPhysicalDevice, instance: Arc<Instance>) -> Self {
        PhysicalDevice { handle, instance }
    }
}
