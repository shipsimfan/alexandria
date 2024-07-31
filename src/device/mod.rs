use std::sync::Arc;

mod new;

pub use new::DeviceCreateError;

/// A instance of a device that can be used for rendering
pub struct Device {
    /// The Vulkan device
    device: Arc<vk::Device>,
}
