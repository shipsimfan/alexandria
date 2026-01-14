use crate::{GraphicsAdapter, GraphicsError, Result, WindowSurface};
use vulkan::{VK_FALSE, VK_TRUE, try_vulkan};

impl<'instance> GraphicsAdapter<'instance> {
    /// Does this adapter support `surface` with `queue_family_index`
    pub fn supports_surface(
        &self,
        queue_family_index: u32,
        surface: &WindowSurface,
    ) -> Result<bool> {
        let mut supported = VK_FALSE;
        try_vulkan!((self
            .instance
            .functions
            .surface()
            .get_physical_device_surface_support)(
            self.handle,
            queue_family_index,
            surface.handle(),
            &mut supported
        ))
        .map(|_| supported == VK_TRUE)
        .map_err(|vk| GraphicsError::new_vk("unable to check if an adapter supports a surface", vk))
    }
}
