use crate::{
    Error, Result,
    gpu::{VulkanDeviceMemory, VulkanImage},
};
use vulkan::try_vulkan;

impl VulkanImage {
    /// Bind this image to a memory object
    pub fn bind_memory(&mut self, memory: &VulkanDeviceMemory, offset: u64) -> Result<()> {
        try_vulkan!((self.inner.device().functions().image.bind_image_memory)(
            self.inner.device().handle(),
            self.inner.handle(),
            memory.handle(),
            offset
        ))
        .map(|_| ())
        .map_err(|error| Error::new_with("unable to bind memory to an image", error))
    }
}
