use crate::device::inner::GraphicsDeviceInner;
use std::sync::Arc;
use vulkan::VkImage;

mod new;

/// An image on the GPU
pub struct GpuImage {
    /// The handle to the underlying image
    handle: VkImage,

    /// The device this image came from
    device: Arc<GraphicsDeviceInner>,
}
