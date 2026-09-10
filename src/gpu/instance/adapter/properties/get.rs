use crate::{
    Uuid,
    gpu::{VulkanAdapterProperties, VulkanAdapterType, VulkanVersion},
    math::Vector2u,
};
use std::{borrow::Cow, ffi::CStr};

impl VulkanAdapterProperties {
    /// Get the API version supported by the adapter
    pub fn api_version(&self) -> VulkanVersion {
        unsafe { VulkanVersion::new_raw(self.inner.api_version) }
    }

    /// Get the driver version of the adapter
    pub fn driver_version(&self) -> VulkanVersion {
        unsafe { VulkanVersion::new_raw(self.inner.driver_version) }
    }

    /// Get the vendor ID of the adapter
    pub fn vendor_id(&self) -> u32 {
        self.inner.vendor_id
    }

    /// Get the device ID of the adapter
    pub fn device_id(&self) -> u32 {
        self.inner.device_id
    }

    /// Get the device type of the adapter
    pub fn device_type(&self) -> VulkanAdapterType {
        self.inner.device_type
    }

    /// Get the device name of the adapter
    pub fn device_name<'a>(&'a self) -> Cow<'a, str> {
        let name = unsafe { CStr::from_ptr(self.inner.device_name.as_ptr()) };
        name.to_string_lossy()
    }

    /// Get the pipeline cache UUID of the adapter
    pub fn pipeline_cache_uuid(&self) -> Uuid {
        Uuid::from_flat(self.inner.pipeline_cache_uuid)
    }

    /// Get the maximum size of push constants supported by the adapter
    pub fn max_push_constants_size(&self) -> u32 {
        self.inner.limits.max_push_constants_size
    }

    /// Get the maximum number of device memory allocations supported by the adapter
    pub fn max_memory_allocations(&self) -> u32 {
        self.inner.limits.max_memory_allocation_count
    }

    /// Get the maximum number of sampler allocations supported by the adapter
    pub fn max_sampler_allocation_count(&self) -> u32 {
        self.inner.limits.max_sampler_allocation_count
    }

    /// Get the buffer image granularity of the adapter
    pub fn buffer_image_granularity(&self) -> u64 {
        self.inner.limits.buffer_image_granularity
    }

    /// Get the maximum number of descriptor sets that can be bound
    pub fn max_bound_descriptor_sets(&self) -> u32 {
        self.inner.limits.max_bound_descriptor_sets
    }

    /// Get the maximum number of samplers per stage supported by the adapter
    pub fn max_per_stage_descriptor_samplers(&self) -> u32 {
        self.inner.limits.max_per_stage_descriptor_samplers
    }

    /// Get the maximum number of uniform buffers per stage supported by the adapter
    pub fn max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
        self.inner.limits.max_per_stage_descriptor_uniform_buffers
    }

    /// Get the maximum number of storage buffers per stage supported by the adapter
    pub fn max_per_stage_descriptor_storage_buffers(&self) -> u32 {
        self.inner.limits.max_per_stage_descriptor_storage_buffers
    }

    /// Get the maximum number of sampled images per stage supported by the adapter
    pub fn max_per_stage_descriptor_sampled_images(&self) -> u32 {
        self.inner.limits.max_per_stage_descriptor_sampled_images
    }

    /// Get the maximum number of storage images per stage supported by the adapter
    pub fn max_per_stage_descriptor_storage_images(&self) -> u32 {
        self.inner.limits.max_per_stage_descriptor_storage_images
    }

    /// Get the maximum number of input attachments per stage supported by the adapter
    pub fn max_per_stage_descriptor_input_attachments(&self) -> u32 {
        self.inner.limits.max_per_stage_descriptor_input_attachments
    }

    /// Get the maximum number of resources per stage supported by the adapter
    pub fn max_per_stage_resources(&self) -> u32 {
        self.inner.limits.max_per_stage_resources
    }

    /// Get the maximum number of samplers in a descriptor set supported by the adapter
    pub fn max_descriptor_set_samplers(&self) -> u32 {
        self.inner.limits.max_descriptor_set_samplers
    }

    /// Get the maximum number of uniform buffers in a descriptor set supported by the adapter
    pub fn max_descriptor_set_uniform_buffers(&self) -> u32 {
        self.inner.limits.max_descriptor_set_uniform_buffers
    }

    /// Get the maximum number of dynamic uniform buffers in a descriptor set supported by the adapter
    pub fn max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
        self.inner.limits.max_descriptor_set_uniform_buffers_dynamic
    }

    /// Get the maximum number of storage buffers in a descriptor set supported by the adapter
    pub fn max_descriptor_set_storage_buffers(&self) -> u32 {
        self.inner.limits.max_descriptor_set_storage_buffers
    }

    /// Get the maximum number of dynamic storage buffers in a descriptor set supported by the adapter
    pub fn max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
        self.inner.limits.max_descriptor_set_storage_buffers_dynamic
    }

    /// Get the maximum number of sampled images in a descriptor set supported by the adapter
    pub fn max_descriptor_set_sampled_images(&self) -> u32 {
        self.inner.limits.max_descriptor_set_sampled_images
    }

    /// Get the maximum number of storage images in a descriptor set supported by the adapter
    pub fn max_descriptor_set_storage_images(&self) -> u32 {
        self.inner.limits.max_descriptor_set_storage_images
    }

    /// Get the maximum number of input attachments in a descriptor set supported by the adapter
    pub fn max_descriptor_set_input_attachments(&self) -> u32 {
        self.inner.limits.max_descriptor_set_input_attachments
    }

    /// Get the maximum number of vertex input attributes supported by the adapter
    pub fn max_vertex_input_attributes(&self) -> u32 {
        self.inner.limits.max_vertex_input_attributes
    }

    /// Get the maximum number of vertex input bindings supported by the adapter
    pub fn max_vertex_input_bindings(&self) -> u32 {
        self.inner.limits.max_vertex_input_bindings
    }

    /// Get the maximum offset supported for vertex input attributes by the adapter
    pub fn max_vertex_input_attribute_offset(&self) -> u32 {
        self.inner.limits.max_vertex_input_attribute_offset
    }

    /// Get the maximum stride supported for vertex input bindings by the adapter
    pub fn max_vertex_input_binding_stride(&self) -> u32 {
        self.inner.limits.max_vertex_input_binding_stride
    }

    /// Get the maximum LOD bias supported for mipmapped images by the adapter
    pub fn max_sampler_lod_bias(&self) -> f32 {
        self.inner.limits.max_sampler_lod_bias
    }

    /// Get the maximum anisotropy supported for samplers by the adapter
    pub fn max_sampler_anisotropy(&self) -> f32 {
        self.inner.limits.max_sampler_anisotropy
    }

    /// Get the maximum number of viewports supported by the adapter
    pub fn max_viewports(&self) -> u32 {
        self.inner.limits.max_viewports
    }

    /// Get the maximum number of viewport dimensions supported by the adapter
    pub fn max_viewport_dimensions(&self) -> Vector2u {
        Vector2u::new(
            self.inner.limits.max_viewport_dimensions[0],
            self.inner.limits.max_viewport_dimensions[1],
        )
    }

    /// Get the minimum memory map alignment supported by the adapter
    pub fn min_memory_map_alignment(&self) -> usize {
        self.inner.limits.min_memory_map_alignment
    }
}
