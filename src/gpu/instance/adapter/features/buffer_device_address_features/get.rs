use crate::gpu::VulkanDeviceBufferDeviceAddressFeatures;
use vulkan::VK_TRUE;

// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanCommandBuffer;

impl VulkanDeviceBufferDeviceAddressFeatures {
    /// Get whether buffer device address is enabled
    pub fn buffer_device_address(&self) -> bool {
        self.inner.buffer_device_address == VK_TRUE
    }

    /// Get whether buffer device address capture replay is enabled
    pub fn buffer_device_address_capture_replay(&self) -> bool {
        self.inner.buffer_device_address_capture_replay == VK_TRUE
    }

    /// Get whether buffer device address multi-device is enabled
    pub fn buffer_device_address_multi_device(&self) -> bool {
        self.inner.buffer_device_address_multi_device == VK_TRUE
    }
}
