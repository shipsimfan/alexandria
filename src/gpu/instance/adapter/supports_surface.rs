use crate::{
    Error, Result,
    gpu::{VulkanAdapter, VulkanSurface},
};
use vulkan::{VK_FALSE, VK_TRUE, try_vulkan};

impl<'instance> VulkanAdapter<'instance> {
    /// Does this adapter support `surface` with `queue_family_index`
    pub fn supports_surface(
        &self,
        queue_family_index: u32,
        surface: &VulkanSurface,
    ) -> Result<bool> {
        let mut supported = VK_FALSE;
        try_vulkan!((self
            .instance
            .functions()
            .surface()
            .get_physical_device_surface_support)(
            self.handle,
            queue_family_index,
            surface.handle(),
            &mut supported
        ))
        .map(|_| supported == VK_TRUE)
        .map_err(|vk| Error::new_with("unable to check if an adapter supports a surface", vk))
    }
}
