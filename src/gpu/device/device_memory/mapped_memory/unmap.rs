use crate::gpu::{VulkanDeviceMemory, VulkanMappedMemory};

impl<T> VulkanMappedMemory<T> {
    /// Unmap the memory from the host
    pub fn unmap(mut self) -> VulkanDeviceMemory {
        self.do_unmap();

        self.memory.take().unwrap()
    }

    /// Actually unmap the memory from the host
    pub(in crate::gpu::device::device_memory::mapped_memory) fn do_unmap(&mut self) {
        if let Some(memory) = &self.memory {
            unsafe {
                (memory.device.functions().device_memory.unmap_memory)(
                    memory.device.handle(),
                    memory.handle(),
                );
            }
        }
    }
}
