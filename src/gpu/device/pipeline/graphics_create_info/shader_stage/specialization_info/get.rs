use crate::gpu::{VulkanSpecializationInfo, VulkanSpecializationMapEntry};
use vulkan::VkSpecializationInfo;

impl<'a> VulkanSpecializationInfo<'a> {
    /// Get the map entries of the specialization info
    pub fn map_entries(&self) -> &'a [VulkanSpecializationMapEntry] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.map_entries.cast(),
                self.inner.map_entry_count as _,
            )
        }
    }

    /// Get the data of the specialization info
    pub fn data(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.inner.data.cast(), self.inner.data_size) }
    }

    /// Get a raw pointer to the inner Vulkan specialization info
    pub(in crate::gpu::device::pipeline::graphics_create_info::shader_stage) fn as_ptr(
        &self,
    ) -> *const VkSpecializationInfo {
        &self.inner
    }
}
