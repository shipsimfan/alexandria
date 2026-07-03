use std::marker::PhantomData;
use vulkan::VkSpecializationInfo;

mod map_entry;

mod get;
mod new;
mod set;

pub use map_entry::*;

/// Specialization info for a shader stage
pub struct VulkanSpecializationInfo<'a> {
    /// The inner representation of the specialization info
    inner: VkSpecializationInfo,

    /// A marker to indicate that the map entries and the data are borrowed for the lifetime `'a`
    _marker: PhantomData<&'a ()>,
}
