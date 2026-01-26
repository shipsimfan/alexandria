use crate::{GpuImage, Swapchain};

impl<'surface> Swapchain<'surface> {
    /// Get access to the images that make up this swapchain
    pub fn images(&self) -> &[GpuImage] {
        &self.images
    }
}
