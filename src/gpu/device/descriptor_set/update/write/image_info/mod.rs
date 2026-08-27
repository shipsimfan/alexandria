use std::marker::PhantomData;
use vulkan::VkDescriptorImageInfo;

mod get;
mod new;
mod set;

/// A Vulkan descriptor image info object
#[repr(transparent)]
pub struct VulkanDescriptorImageInfo<'a> {
    /// The underlying Vulkan descriptor image info
    inner: VkDescriptorImageInfo,

    /// A marker for the lifetimes of the image view and sampler
    _marker: PhantomData<&'a ()>,
}
