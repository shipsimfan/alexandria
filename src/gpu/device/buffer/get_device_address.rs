use crate::gpu::{GpuAddress, VulkanBuffer};
use vulkan::VkBufferDeviceAddressInfo;

impl VulkanBuffer {
    /// Get the GPU address of the buffer
    pub fn get_device_address<T>(&self) -> GpuAddress<T> {
        let address_info = VkBufferDeviceAddressInfo {
            buffer: self.handle,
            ..Default::default()
        };
        let address = unsafe {
            (self.device.functions().buffer.get_buffer_device_address)(
                self.device.handle(),
                &address_info,
            )
        };

        GpuAddress::new(address)
    }
}
