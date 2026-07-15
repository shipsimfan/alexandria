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

    /// Get the maximum number of device memory allocations supported by the adapter
    pub fn max_memory_allocations(&self) -> u32 {
        self.inner.limits.max_memory_allocation_count
    }

    /// Get the buffer image granularity of the adapter
    pub fn buffer_image_granularity(&self) -> u64 {
        self.inner.limits.buffer_image_granularity
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
