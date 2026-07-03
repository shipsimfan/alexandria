use vulkan::VkSpecializationMapEntry;

mod get;
mod new;
mod set;

/// An entry in the specialization map for a shader stage
#[repr(transparent)]
pub struct VulkanSpecializationMapEntry {
    /// The inner Vulkan specialization map entry
    inner: VkSpecializationMapEntry,
}
