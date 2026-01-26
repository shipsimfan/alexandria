use crate::{GpuImage, device::GraphicsDeviceInner};
use std::sync::Arc;
use vulkan::VkImage;

impl GpuImage {
    /// Create a new [`GpuImage`]
    pub(in crate::device) fn new(handle: VkImage, device: Arc<GraphicsDeviceInner>) -> GpuImage {
        GpuImage { handle, device }
    }
}
