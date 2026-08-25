use vulkan::VkImageBlit;

mod get;
mod new;
mod set;

/// The description of a region of an image to blit
#[repr(transparent)]
pub struct VulkanImageBlit {
    /// The inner Vulkan structure representing the image blit region
    inner: VkImageBlit,
}
