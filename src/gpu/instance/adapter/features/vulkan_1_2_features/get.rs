use crate::gpu::VulkanDeviceVulkan12Features;
use vulkan::VK_TRUE;

impl VulkanDeviceVulkan12Features {
    /// Get whether support for mirrored clamp-to-edge sampling is enabled
    pub fn sampler_mirror_clamp_to_edge(&self) -> bool {
        self.inner.sampler_mirror_clamp_to_edge == VK_TRUE
    }

    /// Get whether support for indirect drawing with draw count is enabled
    pub fn draw_indirect_count(&self) -> bool {
        self.inner.draw_indirect_count == VK_TRUE
    }

    /// Get whether support for 8-bit access to storage buffers is enabled
    pub fn storage_buffer_8_bit_access(&self) -> bool {
        self.inner.storage_buffer_8_bit_access == VK_TRUE
    }

    /// Get whether support for 8-bit access to uniform and storage buffers is enabled
    pub fn uniform_and_storage_buffer_8_bit_access(&self) -> bool {
        self.inner.uniform_and_storage_buffer_8_bit_access == VK_TRUE
    }

    /// Get whether push constants support 8-bit types
    pub fn storage_push_constant_8(&self) -> bool {
        self.inner.storage_push_constant_8 == VK_TRUE
    }

    /// Get whether support for 64-bit buffer atomics is enabled
    pub fn shader_buffer_int64_atomics(&self) -> bool {
        self.inner.shader_buffer_int64_atomics == VK_TRUE
    }

    /// Get whether support for 64-bit shared memory atomics is enabled
    pub fn shader_shared_int64_atomics(&self) -> bool {
        self.inner.shader_shared_int64_atomics == VK_TRUE
    }

    /// Get whether support for 16-bit floating-point shader operations is enabled
    pub fn shader_float_16(&self) -> bool {
        self.inner.shader_float_16 == VK_TRUE
    }

    /// Get whether support for 8-bit integer shader operations is enabled
    pub fn shader_int_8(&self) -> bool {
        self.inner.shader_int_8 == VK_TRUE
    }

    /// Get whether support for descriptor indexing is enabled
    pub fn descriptor_indexing(&self) -> bool {
        self.inner.descriptor_indexing == VK_TRUE
    }

    /// Get whether support for dynamic indexing of input attachment arrays in shaders is enabled
    pub fn shader_input_attachment_array_dynamic_indexing(&self) -> bool {
        self.inner.shader_input_attachment_array_dynamic_indexing == VK_TRUE
    }

    /// Get whether support for dynamic indexing of uniform texel buffer arrays in shaders is enabled
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(&self) -> bool {
        self.inner
            .shader_uniform_texel_buffer_array_dynamic_indexing
            == VK_TRUE
    }

    /// Get whether support for dynamic indexing of storage texel buffer arrays in shaders is enabled
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(&self) -> bool {
        self.inner
            .shader_storage_texel_buffer_array_dynamic_indexing
            == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of uniform buffer arrays in shaders is enabled
    pub fn shader_uniform_buffer_array_non_uniform_indexing(&self) -> bool {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of sampled image arrays in shaders is enabled
    pub fn shader_sampled_image_array_non_uniform_indexing(&self) -> bool {
        self.inner.shader_sampled_image_array_non_uniform_indexing == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of storage buffer arrays in shaders is enabled
    pub fn shader_storage_buffer_array_non_uniform_indexing(&self) -> bool {
        self.inner.shader_storage_buffer_array_non_uniform_indexing == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of storage image arrays in shaders is enabled
    pub fn shader_storage_image_array_non_uniform_indexing(&self) -> bool {
        self.inner.shader_storage_image_array_non_uniform_indexing == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of input attachment arrays in shaders is enabled
    pub fn shader_input_attachment_array_non_uniform_indexing(&self) -> bool {
        self.inner
            .shader_input_attachment_array_non_uniform_indexing
            == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of uniform texel buffer arrays in shaders is enabled
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(&self) -> bool {
        self.inner
            .shader_uniform_texel_buffer_array_non_uniform_indexing
            == VK_TRUE
    }

    /// Get whether support for non-uniform indexing of storage texel buffer arrays in shaders is enabled
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(&self) -> bool {
        self.inner
            .shader_storage_texel_buffer_array_non_uniform_indexing
            == VK_TRUE
    }

    /// Get whether support for updating uniform buffers after bind is enabled
    pub fn descriptor_binding_uniform_buffer_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_uniform_buffer_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for updating sampled images after bind is enabled
    pub fn descriptor_binding_sampled_image_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_sampled_image_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for updating storage images after bind is enabled
    pub fn descriptor_binding_storage_image_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_storage_image_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for updating storage buffers after bind is enabled
    pub fn descriptor_binding_storage_buffer_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_storage_buffer_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for updating uniform texel buffers after bind is enabled
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_uniform_texel_buffer_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for updating storage texel buffers after bind is enabled
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_storage_texel_buffer_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for updating descriptors while pending is enabled
    pub fn descriptor_binding_update_unused_while_pending(&self) -> bool {
        self.inner.descriptor_binding_update_unused_while_pending == VK_TRUE
    }

    /// Get whether support for partially-bound descriptors is enabled
    pub fn descriptor_binding_partially_bound(&self) -> bool {
        self.inner.descriptor_binding_partially_bound == VK_TRUE
    }

    /// Get whether support for variable descriptor counts is enabled
    pub fn descriptor_binding_variable_descriptor_count(&self) -> bool {
        self.inner.descriptor_binding_variable_descriptor_count == VK_TRUE
    }

    /// Get whether support for runtime descriptor arrays is enabled
    pub fn runtime_descriptor_array(&self) -> bool {
        self.inner.runtime_descriptor_array == VK_TRUE
    }

    /// Get whether support for minmax filtering in samplers is enabled
    pub fn sampler_filter_minmax(&self) -> bool {
        self.inner.sampler_filter_minmax == VK_TRUE
    }

    /// Get whether support for scalar block layout is enabled
    pub fn scalar_block_layout(&self) -> bool {
        self.inner.scalar_block_layout == VK_TRUE
    }

    /// Get whether support for imageless framebuffers is enabled
    pub fn imageless_framebuffer(&self) -> bool {
        self.inner.imageless_framebuffer == VK_TRUE
    }

    /// Get whether support for standard layout uniform buffers is enabled
    pub fn uniform_buffer_standard_layout(&self) -> bool {
        self.inner.uniform_buffer_standard_layout == VK_TRUE
    }

    /// Get whether support for extended subgroup types is enabled
    pub fn shader_subgroup_extended_types(&self) -> bool {
        self.inner.shader_subgroup_extended_types == VK_TRUE
    }

    /// Get whether support for separate depth and stencil layouts is enabled
    pub fn separate_depth_stencil_layouts(&self) -> bool {
        self.inner.separate_depth_stencil_layouts == VK_TRUE
    }

    /// Get whether host query reset is supported
    pub fn host_query_reset(&self) -> bool {
        self.inner.host_query_reset == VK_TRUE
    }

    /// Get whether support for timeline semaphores is enabled
    pub fn timeline_semaphore(&self) -> bool {
        self.inner.timeline_semaphore == VK_TRUE
    }

    /// Get whether support for buffer device addresses is enabled
    pub fn buffer_device_address(&self) -> bool {
        self.inner.buffer_device_address == VK_TRUE
    }

    /// Get whether support for buffer device address capture and replay is enabled
    pub fn buffer_device_address_capture_replay(&self) -> bool {
        self.inner.buffer_device_address_capture_replay == VK_TRUE
    }

    /// Get whether support for buffer device addresses across multiple devices is enabled
    pub fn buffer_device_address_multi_device(&self) -> bool {
        self.inner.buffer_device_address_multi_device == VK_TRUE
    }

    /// Get whether support for the Vulkan memory model is enabled
    pub fn vulkan_memory_model(&self) -> bool {
        self.inner.vulkan_memory_model == VK_TRUE
    }

    /// Get whether support for Vulkan memory model device scope is enabled
    pub fn vulkan_memory_model_device_scope(&self) -> bool {
        self.inner.vulkan_memory_model_device_scope == VK_TRUE
    }

    /// Get whether support for Vulkan memory model availability and visibility chains is enabled
    pub fn vulkan_memory_model_availability_visibility_chains(&self) -> bool {
        self.inner
            .vulkan_memory_model_availability_visibility_chains
            == VK_TRUE
    }

    /// Get whether support for viewport index output in shaders is enabled
    pub fn shader_output_viewport_index(&self) -> bool {
        self.inner.shader_output_viewport_index == VK_TRUE
    }

    /// Get whether support for layer output in shaders is enabled
    pub fn shader_output_layer(&self) -> bool {
        self.inner.shader_output_layer == VK_TRUE
    }

    /// Get whether support for subgroup broadcast dynamic IDs is enabled
    pub fn subgroup_broadcast_dynamic_id(&self) -> bool {
        self.inner.subgroup_broadcast_dynamic_id == VK_TRUE
    }
}
