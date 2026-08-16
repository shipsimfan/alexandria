use crate::gpu::VulkanAdapterFeature;
use vulkan::VkPhysicalDeviceBufferDeviceAddressFeatures;

mod get;
mod next_chain;
mod set;

/// Buffer device address features
#[derive(Default, Clone)]
pub struct VulkanDeviceBufferDeviceAddressFeatures {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceBufferDeviceAddressFeatures,
}

impl VulkanAdapterFeature for VulkanDeviceBufferDeviceAddressFeatures {}
