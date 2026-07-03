use crate::gpu::{VulkanSpecializationInfo, VulkanSpecializationMapEntry};
use std::marker::PhantomData;
use vulkan::VkSpecializationInfo;

impl<'a> VulkanSpecializationInfo<'a> {
    /// Create a new [`VulkanSpecializationInfo`]
    pub fn new(
        map_entries: &'a [VulkanSpecializationMapEntry],
        data: &'a [u8],
    ) -> VulkanSpecializationInfo<'a> {
        let inner = VkSpecializationInfo {
            map_entry_count: map_entries.len() as _,
            map_entries: map_entries.as_ptr().cast(),
            data_size: data.len(),
            data: data.as_ptr().cast(),
        };

        VulkanSpecializationInfo {
            inner,
            _marker: PhantomData,
        }
    }
}
