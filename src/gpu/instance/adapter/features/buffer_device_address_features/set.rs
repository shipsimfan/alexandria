use crate::gpu::VulkanDeviceBufferDeviceAddressFeatures;
use vulkan::{VK_FALSE, VK_TRUE};

// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanCommandBuffer;

impl VulkanDeviceBufferDeviceAddressFeatures {
    /// Enable buffer device address
    pub fn enable_buffer_device_address(mut self) -> Self {
        self.inner.buffer_device_address = VK_TRUE;
        self
    }

    /// Disable buffer device address
    pub fn disable_buffer_device_address(mut self) -> Self {
        self.inner.buffer_device_address = VK_FALSE;
        self
    }

    /// Enable buffer device address capture replay
    pub fn enable_buffer_device_address_capture_replay(mut self) -> Self {
        self.inner.buffer_device_address_capture_replay = VK_TRUE;
        self
    }

    /// Disable buffer device address capture replay
    pub fn disable_buffer_device_address_capture_replay(mut self) -> Self {
        self.inner.buffer_device_address_capture_replay = VK_FALSE;
        self
    }

    /// Enable buffer device address multi-device
    pub fn enable_buffer_device_address_multi_device(mut self) -> Self {
        self.inner.buffer_device_address_multi_device = VK_TRUE;
        self
    }

    /// Disable buffer device address multi-device
    pub fn disable_buffer_device_address_multi_device(mut self) -> Self {
        self.inner.buffer_device_address_multi_device = VK_FALSE;
        self
    }
}
