use super::PhysicalDevice;
use crate::Instance;
use std::rc::Rc;
use vulkan::VkPhysicalDevice;

impl PhysicalDevice {
    /// Creates a new [`PhysicalDevice`]
    pub(crate) fn new(handle: VkPhysicalDevice, instance: Rc<Instance>) -> Self {
        PhysicalDevice { handle, instance }
    }
}
