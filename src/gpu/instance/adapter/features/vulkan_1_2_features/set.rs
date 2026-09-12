use crate::gpu::VulkanDeviceVulkan12Features;
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanDeviceVulkan12Features {
    /// Enable support for mirrored clamp-to-edge sampling
    pub fn enable_sampler_mirror_clamp_to_edge(mut self) -> Self {
        self.inner.sampler_mirror_clamp_to_edge = VK_TRUE;
        self
    }

    /// Disable support for mirrored clamp-to-edge sampling
    pub fn disable_sampler_mirror_clamp_to_edge(mut self) -> Self {
        self.inner.sampler_mirror_clamp_to_edge = VK_FALSE;
        self
    }

    /// Enable support for indirect drawing with draw count
    pub fn enable_draw_indirect_count(mut self) -> Self {
        self.inner.draw_indirect_count = VK_TRUE;
        self
    }

    /// Disable support for indirect drawing with draw count
    pub fn disable_draw_indirect_count(mut self) -> Self {
        self.inner.draw_indirect_count = VK_FALSE;
        self
    }

    /// Enable support for 8-bit access to storage buffers
    pub fn enable_storage_buffer_8_bit_access(mut self) -> Self {
        self.inner.storage_buffer_8_bit_access = VK_TRUE;
        self
    }

    /// Disable support for 8-bit access to storage buffers
    pub fn disable_storage_buffer_8_bit_access(mut self) -> Self {
        self.inner.storage_buffer_8_bit_access = VK_FALSE;
        self
    }

    /// Enable support for 8-bit access to uniform and storage buffers
    pub fn enable_uniform_and_storage_buffer_8_bit_access(mut self) -> Self {
        self.inner.uniform_and_storage_buffer_8_bit_access = VK_TRUE;
        self
    }

    /// Disable support for 8-bit access to uniform and storage buffers
    pub fn disable_uniform_and_storage_buffer_8_bit_access(mut self) -> Self {
        self.inner.uniform_and_storage_buffer_8_bit_access = VK_FALSE;
        self
    }

    /// Enable push constants support 8-bit types
    pub fn enable_storage_push_constant_8(mut self) -> Self {
        self.inner.storage_push_constant_8 = VK_TRUE;
        self
    }

    /// Disable push constants support 8-bit types
    pub fn disable_storage_push_constant_8(mut self) -> Self {
        self.inner.storage_push_constant_8 = VK_FALSE;
        self
    }

    /// Enable support for 64-bit buffer atomics
    pub fn enable_shader_buffer_int64_atomics(mut self) -> Self {
        self.inner.shader_buffer_int64_atomics = VK_TRUE;
        self
    }

    /// Disable support for 64-bit buffer atomics
    pub fn disable_shader_buffer_int64_atomics(mut self) -> Self {
        self.inner.shader_buffer_int64_atomics = VK_FALSE;
        self
    }

    /// Enable support for 64-bit shared memory atomics
    pub fn enable_shader_shared_int64_atomics(mut self) -> Self {
        self.inner.shader_shared_int64_atomics = VK_TRUE;
        self
    }

    /// Disable support for 64-bit shared memory atomics
    pub fn disable_shader_shared_int64_atomics(mut self) -> Self {
        self.inner.shader_shared_int64_atomics = VK_FALSE;
        self
    }

    /// Enable support for 16-bit floating-point shader operations
    pub fn enable_shader_float_16(mut self) -> Self {
        self.inner.shader_float_16 = VK_TRUE;
        self
    }

    /// Disable support for 16-bit floating-point shader operations
    pub fn disable_shader_float_16(mut self) -> Self {
        self.inner.shader_float_16 = VK_FALSE;
        self
    }

    /// Enable support for 8-bit integer shader operations
    pub fn enable_shader_int_8(mut self) -> Self {
        self.inner.shader_int_8 = VK_TRUE;
        self
    }

    /// Disable support for 8-bit integer shader operations
    pub fn disable_shader_int_8(mut self) -> Self {
        self.inner.shader_int_8 = VK_FALSE;
        self
    }

    /// Enable support for descriptor indexing
    pub fn enable_descriptor_indexing(mut self) -> Self {
        self.inner.descriptor_indexing = VK_TRUE;
        self
    }

    /// Disable support for descriptor indexing
    pub fn disable_descriptor_indexing(mut self) -> Self {
        self.inner.descriptor_indexing = VK_FALSE;
        self
    }

    /// Enable support for dynamic indexing of input attachment arrays in shaders
    pub fn enable_shader_input_attachment_array_dynamic_indexing(mut self) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable support for dynamic indexing of input attachment arrays in shaders
    pub fn disable_shader_input_attachment_array_dynamic_indexing(mut self) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable support for dynamic indexing of uniform texel buffer arrays in shaders
    pub fn enable_shader_uniform_texel_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .shader_uniform_texel_buffer_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable support for dynamic indexing of uniform texel buffer arrays in shaders
    pub fn disable_shader_uniform_texel_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .shader_uniform_texel_buffer_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable support for dynamic indexing of storage texel buffer arrays in shaders
    pub fn enable_shader_storage_texel_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .shader_storage_texel_buffer_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable support for dynamic indexing of storage texel buffer arrays in shaders
    pub fn disable_shader_storage_texel_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .shader_storage_texel_buffer_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of uniform buffer arrays in shaders
    pub fn enable_shader_uniform_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of uniform buffer arrays in shaders
    pub fn disable_shader_uniform_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of sampled image arrays in shaders
    pub fn enable_shader_sampled_image_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of sampled image arrays in shaders
    pub fn disable_shader_sampled_image_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of storage buffer arrays in shaders
    pub fn enable_shader_storage_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of storage buffer arrays in shaders
    pub fn disable_shader_storage_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of storage image arrays in shaders
    pub fn enable_shader_storage_image_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of storage image arrays in shaders
    pub fn disable_shader_storage_image_array_non_uniform_indexing(mut self) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of input attachment arrays in shaders
    pub fn enable_shader_input_attachment_array_non_uniform_indexing(mut self) -> Self {
        self.inner
            .shader_input_attachment_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of input attachment arrays in shaders
    pub fn disable_shader_input_attachment_array_non_uniform_indexing(mut self) -> Self {
        self.inner
            .shader_input_attachment_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of uniform texel buffer arrays in shaders
    pub fn enable_shader_uniform_texel_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner
            .shader_uniform_texel_buffer_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of uniform texel buffer arrays in shaders
    pub fn disable_shader_uniform_texel_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner
            .shader_uniform_texel_buffer_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for non-uniform indexing of storage texel buffer arrays in shaders
    pub fn enable_shader_storage_texel_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner
            .shader_storage_texel_buffer_array_non_uniform_indexing = VK_TRUE;
        self
    }

    /// Disable support for non-uniform indexing of storage texel buffer arrays in shaders
    pub fn disable_shader_storage_texel_buffer_array_non_uniform_indexing(mut self) -> Self {
        self.inner
            .shader_storage_texel_buffer_array_non_uniform_indexing = VK_FALSE;
        self
    }

    /// Enable support for updating uniform buffers after bind
    pub fn enable_descriptor_binding_uniform_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_uniform_buffer_update_after_bind = VK_TRUE;
        self
    }

    /// Disable support for updating uniform buffers after bind
    pub fn disable_descriptor_binding_uniform_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_uniform_buffer_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for updating sampled images after bind
    pub fn enable_descriptor_binding_sampled_image_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_sampled_image_update_after_bind = VK_TRUE;
        self
    }

    /// Disable support for updating sampled images after bind
    pub fn disable_descriptor_binding_sampled_image_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_sampled_image_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for updating storage images after bind
    pub fn enable_descriptor_binding_storage_image_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_storage_image_update_after_bind = VK_TRUE;
        self
    }

    /// Disable support for updating storage images after bind
    pub fn disable_descriptor_binding_storage_image_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_storage_image_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for updating storage buffers after bind
    pub fn enable_descriptor_binding_storage_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_storage_buffer_update_after_bind = VK_TRUE;
        self
    }

    /// Disable support for updating storage buffers after bind
    pub fn disable_descriptor_binding_storage_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_storage_buffer_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for updating uniform texel buffers after bind
    pub fn enable_descriptor_binding_uniform_texel_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_uniform_texel_buffer_update_after_bind = VK_TRUE;
        self
    }

    /// Disable support for updating uniform texel buffers after bind
    pub fn disable_descriptor_binding_uniform_texel_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_uniform_texel_buffer_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for updating storage texel buffers after bind
    pub fn enable_descriptor_binding_storage_texel_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_storage_texel_buffer_update_after_bind = VK_TRUE;
        self
    }

    /// Disable support for updating storage texel buffers after bind
    pub fn disable_descriptor_binding_storage_texel_buffer_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_storage_texel_buffer_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for updating descriptors while pending
    pub fn enable_descriptor_binding_update_unused_while_pending(mut self) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = VK_TRUE;
        self
    }

    /// Disable support for updating descriptors while pending
    pub fn disable_descriptor_binding_update_unused_while_pending(mut self) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = VK_FALSE;
        self
    }

    /// Enable support for partially-bound descriptors
    pub fn enable_descriptor_binding_partially_bound(mut self) -> Self {
        self.inner.descriptor_binding_partially_bound = VK_TRUE;
        self
    }

    /// Disable support for partially-bound descriptors
    pub fn disable_descriptor_binding_partially_bound(mut self) -> Self {
        self.inner.descriptor_binding_partially_bound = VK_FALSE;
        self
    }

    /// Enable support for variable descriptor counts
    pub fn enable_descriptor_binding_variable_descriptor_count(mut self) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = VK_TRUE;
        self
    }

    /// Disable support for variable descriptor counts
    pub fn disable_descriptor_binding_variable_descriptor_count(mut self) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = VK_FALSE;
        self
    }

    /// Enable support for runtime descriptor arrays
    pub fn enable_runtime_descriptor_array(mut self) -> Self {
        self.inner.runtime_descriptor_array = VK_TRUE;
        self
    }

    /// Disable support for runtime descriptor arrays
    pub fn disable_runtime_descriptor_array(mut self) -> Self {
        self.inner.runtime_descriptor_array = VK_FALSE;
        self
    }

    /// Enable support for minmax filtering in samplers
    pub fn enable_sampler_filter_minmax(mut self) -> Self {
        self.inner.sampler_filter_minmax = VK_TRUE;
        self
    }

    /// Disable support for minmax filtering in samplers
    pub fn disable_sampler_filter_minmax(mut self) -> Self {
        self.inner.sampler_filter_minmax = VK_FALSE;
        self
    }

    /// Enable support for scalar block layout
    pub fn enable_scalar_block_layout(mut self) -> Self {
        self.inner.scalar_block_layout = VK_TRUE;
        self
    }

    /// Disable support for scalar block layout
    pub fn disable_scalar_block_layout(mut self) -> Self {
        self.inner.scalar_block_layout = VK_FALSE;
        self
    }

    /// Enable support for imageless framebuffers
    pub fn enable_imageless_framebuffer(mut self) -> Self {
        self.inner.imageless_framebuffer = VK_TRUE;
        self
    }

    /// Disable support for imageless framebuffers
    pub fn disable_imageless_framebuffer(mut self) -> Self {
        self.inner.imageless_framebuffer = VK_FALSE;
        self
    }

    /// Enable support for standard layout uniform buffers
    pub fn enable_uniform_buffer_standard_layout(mut self) -> Self {
        self.inner.uniform_buffer_standard_layout = VK_TRUE;
        self
    }

    /// Disable support for standard layout uniform buffers
    pub fn disable_uniform_buffer_standard_layout(mut self) -> Self {
        self.inner.uniform_buffer_standard_layout = VK_FALSE;
        self
    }

    /// Enable support for extended subgroup types
    pub fn enable_shader_subgroup_extended_types(mut self) -> Self {
        self.inner.shader_subgroup_extended_types = VK_TRUE;
        self
    }

    /// Disable support for extended subgroup types
    pub fn disable_shader_subgroup_extended_types(mut self) -> Self {
        self.inner.shader_subgroup_extended_types = VK_FALSE;
        self
    }

    /// Enable support for separate depth and stencil layouts
    pub fn enable_separate_depth_stencil_layouts(mut self) -> Self {
        self.inner.separate_depth_stencil_layouts = VK_TRUE;
        self
    }

    /// Disable support for separate depth and stencil layouts
    pub fn disable_separate_depth_stencil_layouts(mut self) -> Self {
        self.inner.separate_depth_stencil_layouts = VK_FALSE;
        self
    }

    /// Enable host query reset
    pub fn enable_host_query_reset(mut self) -> Self {
        self.inner.host_query_reset = VK_TRUE;
        self
    }

    /// Disable host query reset
    pub fn disable_host_query_reset(mut self) -> Self {
        self.inner.host_query_reset = VK_FALSE;
        self
    }

    /// Enable support for timeline semaphores
    pub fn enable_timeline_semaphore(mut self) -> Self {
        self.inner.timeline_semaphore = VK_TRUE;
        self
    }

    /// Disable support for timeline semaphores
    pub fn disable_timeline_semaphore(mut self) -> Self {
        self.inner.timeline_semaphore = VK_FALSE;
        self
    }

    /// Enable support for buffer device addresses
    pub fn enable_buffer_device_address(mut self) -> Self {
        self.inner.buffer_device_address = VK_TRUE;
        self
    }

    /// Disable support for buffer device addresses
    pub fn disable_buffer_device_address(mut self) -> Self {
        self.inner.buffer_device_address = VK_FALSE;
        self
    }

    /// Enable support for buffer device address capture and replay
    pub fn enable_buffer_device_address_capture_replay(mut self) -> Self {
        self.inner.buffer_device_address_capture_replay = VK_TRUE;
        self
    }

    /// Disable support for buffer device address capture and replay
    pub fn disable_buffer_device_address_capture_replay(mut self) -> Self {
        self.inner.buffer_device_address_capture_replay = VK_FALSE;
        self
    }

    /// Enable support for buffer device addresses across multiple devices
    pub fn enable_buffer_device_address_multi_device(mut self) -> Self {
        self.inner.buffer_device_address_multi_device = VK_TRUE;
        self
    }

    /// Disable support for buffer device addresses across multiple devices
    pub fn disable_buffer_device_address_multi_device(mut self) -> Self {
        self.inner.buffer_device_address_multi_device = VK_FALSE;
        self
    }

    /// Enable support for the Vulkan memory model
    pub fn enable_vulkan_memory_model(mut self) -> Self {
        self.inner.vulkan_memory_model = VK_TRUE;
        self
    }

    /// Disable support for the Vulkan memory model
    pub fn disable_vulkan_memory_model(mut self) -> Self {
        self.inner.vulkan_memory_model = VK_FALSE;
        self
    }

    /// Enable support for Vulkan memory model device scope
    pub fn enable_vulkan_memory_model_device_scope(mut self) -> Self {
        self.inner.vulkan_memory_model_device_scope = VK_TRUE;
        self
    }

    /// Disable support for Vulkan memory model device scope
    pub fn disable_vulkan_memory_model_device_scope(mut self) -> Self {
        self.inner.vulkan_memory_model_device_scope = VK_FALSE;
        self
    }

    /// Enable support for Vulkan memory model availability and visibility chains
    pub fn enable_vulkan_memory_model_availability_visibility_chains(mut self) -> Self {
        self.inner
            .vulkan_memory_model_availability_visibility_chains = VK_TRUE;
        self
    }

    /// Disable support for Vulkan memory model availability and visibility chains
    pub fn disable_vulkan_memory_model_availability_visibility_chains(mut self) -> Self {
        self.inner
            .vulkan_memory_model_availability_visibility_chains = VK_FALSE;
        self
    }

    /// Enable support for viewport index output in shaders
    pub fn enable_shader_output_viewport_index(mut self) -> Self {
        self.inner.shader_output_viewport_index = VK_TRUE;
        self
    }

    /// Disable support for viewport index output in shaders
    pub fn disable_shader_output_viewport_index(mut self) -> Self {
        self.inner.shader_output_viewport_index = VK_FALSE;
        self
    }

    /// Enable support for layer output in shaders
    pub fn enable_shader_output_layer(mut self) -> Self {
        self.inner.shader_output_layer = VK_TRUE;
        self
    }

    /// Disable support for layer output in shaders
    pub fn disable_shader_output_layer(mut self) -> Self {
        self.inner.shader_output_layer = VK_FALSE;
        self
    }

    /// Enable support for subgroup broadcast dynamic IDs
    pub fn enable_subgroup_broadcast_dynamic_id(mut self) -> Self {
        self.inner.subgroup_broadcast_dynamic_id = VK_TRUE;
        self
    }

    /// Disable support for subgroup broadcast dynamic IDs
    pub fn disable_subgroup_broadcast_dynamic_id(mut self) -> Self {
        self.inner.subgroup_broadcast_dynamic_id = VK_FALSE;
        self
    }
}
