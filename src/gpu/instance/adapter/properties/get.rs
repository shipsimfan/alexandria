use crate::{
    Uuid,
    gpu::{VulkanAdapterProperties, VulkanAdapterType, VulkanSampleCountFlags, VulkanVersion},
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

    /// Get the maximum number of vertex shader output components supported by the adapter
    pub fn max_vertex_output_components(&self) -> u32 {
        self.inner.limits.max_vertex_output_components
    }

    /// Get the maximum tessellation generation level supported by the adapter
    pub fn max_tessellation_generation_level(&self) -> u32 {
        self.inner.limits.max_tessellation_generation_level
    }

    /// Get the maximum tessellation patch size supported by the adapter
    pub fn max_tessellation_patch_size(&self) -> u32 {
        self.inner.limits.max_tessellation_patch_size
    }

    /// Get the maximum per-vertex tessellation input components supported by the adapter
    pub fn max_tessellation_control_per_vertex_input_components(&self) -> u32 {
        self.inner
            .limits
            .max_tessellation_control_per_vertex_input_components
    }

    /// Get the maximum per-vertex tessellation output components supported by the adapter
    pub fn max_tessellation_control_per_vertex_output_components(&self) -> u32 {
        self.inner
            .limits
            .max_tessellation_control_per_vertex_output_components
    }

    /// Get the maximum per-patch tessellation output components supported by the adapter
    pub fn max_tessellation_control_per_patch_output_components(&self) -> u32 {
        self.inner
            .limits
            .max_tessellation_control_per_patch_output_components
    }

    /// Get the maximum total tessellation output components supported by the adapter
    pub fn max_tessellation_control_total_output_components(&self) -> u32 {
        self.inner
            .limits
            .max_tessellation_control_total_output_components
    }

    /// Get the maximum tessellation evaluation input components supported by the adapter
    pub fn max_tessellation_evaluation_input_components(&self) -> u32 {
        self.inner
            .limits
            .max_tessellation_evaluation_input_components
    }

    /// Get the maximum tessellation evaluation output components supported by the adapter
    pub fn max_tessellation_evaluation_output_components(&self) -> u32 {
        self.inner
            .limits
            .max_tessellation_evaluation_output_components
    }

    /// Get the maximum geometry shader invocation count supported by the adapter
    pub fn max_geometry_shader_invocations(&self) -> u32 {
        self.inner.limits.max_geometry_shader_invocations
    }

    /// Get the maximum geometry shader input components supported by the adapter
    pub fn max_geometry_input_components(&self) -> u32 {
        self.inner.limits.max_geometry_input_components
    }

    /// Get the maximum geometry shader output components supported by the adapter
    pub fn max_geometry_output_components(&self) -> u32 {
        self.inner.limits.max_geometry_output_components
    }

    /// Get the maximum geometry shader output vertices supported by the adapter
    pub fn max_geometry_output_vertices(&self) -> u32 {
        self.inner.limits.max_geometry_output_vertices
    }

    /// Get the maximum total geometry shader output components supported by the adapter
    pub fn max_geometry_total_output_components(&self) -> u32 {
        self.inner.limits.max_geometry_total_output_components
    }

    /// Get the maximum fragment shader input components supported by the adapter
    pub fn max_fragment_input_components(&self) -> u32 {
        self.inner.limits.max_fragment_input_components
    }

    /// Get the maximum fragment shader output attachments supported by the adapter
    pub fn max_fragment_output_attachments(&self) -> u32 {
        self.inner.limits.max_fragment_output_attachments
    }

    /// Get the maximum dual-source fragment shader output attachments supported by the adapter
    pub fn max_fragment_dual_src_attachments(&self) -> u32 {
        self.inner.limits.max_fragment_dual_src_attachments
    }

    /// Get the maximum combined fragment output resources supported by the adapter
    pub fn max_fragment_combined_output_resources(&self) -> u32 {
        self.inner.limits.max_fragment_combined_output_resources
    }

    /// Get the maximum shared memory size for compute shaders supported by the adapter
    pub fn max_compute_shared_memory_size(&self) -> u32 {
        self.inner.limits.max_compute_shared_memory_size
    }

    /// Get the maximum number of compute work groups supported by the adapter
    pub fn max_compute_work_group_count(&self) -> [u32; 3] {
        self.inner.limits.max_compute_work_group_count
    }

    /// Get the maximum compute shader invocation count supported by the adapter
    pub fn max_compute_work_group_invocations(&self) -> u32 {
        self.inner.limits.max_compute_work_group_invocations
    }

    /// Get the maximum compute work group size supported by the adapter
    pub fn max_compute_work_group_size(&self) -> [u32; 3] {
        self.inner.limits.max_compute_work_group_size
    }

    /// Get the sub-pixel precision bits supported by the adapter
    pub fn sub_pixel_precision_bits(&self) -> u32 {
        self.inner.limits.sub_pixel_precision_bits
    }

    /// Get the sub-texel precision bits supported by the adapter
    pub fn sub_texel_precision_bits(&self) -> u32 {
        self.inner.limits.sub_texel_precision_bits
    }

    /// Get the mipmap precision bits supported by the adapter
    pub fn mipmap_precision_bits(&self) -> u32 {
        self.inner.limits.mipmap_precision_bits
    }

    /// Get the maximum indexed draw value supported by the adapter
    pub fn max_draw_indexed_index_value(&self) -> u32 {
        self.inner.limits.max_draw_indexed_index_value
    }

    /// Get the maximum draw indirect count supported by the adapter
    pub fn max_draw_indirect_count(&self) -> u32 {
        self.inner.limits.max_draw_indirect_count
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

    /// Get the viewport bounds range supported by the adapter
    pub fn viewport_bounds_range(&self) -> [f32; 2] {
        self.inner.limits.viewport_bounds_range
    }

    /// Get the viewport sub-pixel bits supported by the adapter
    pub fn viewport_sub_pixel_bits(&self) -> u32 {
        self.inner.limits.viewport_sub_pixel_bits
    }

    /// Get the minimum memory map alignment supported by the adapter
    pub fn min_memory_map_alignment(&self) -> usize {
        self.inner.limits.min_memory_map_alignment
    }

    /// Get the minimum texel buffer offset alignment supported by the adapter
    pub fn min_texel_buffer_offset_alignment(&self) -> u64 {
        self.inner.limits.min_texel_buffer_offset_alignment
    }

    /// Get the minimum uniform buffer offset alignment supported by the adapter
    pub fn min_uniform_buffer_offset_alignment(&self) -> u64 {
        self.inner.limits.min_uniform_buffer_offset_alignment
    }

    /// Get the minimum storage buffer offset alignment supported by the adapter
    pub fn min_storage_buffer_offset_alignment(&self) -> u64 {
        self.inner.limits.min_storage_buffer_offset_alignment
    }

    /// Get the minimum texel offset supported by the adapter
    pub fn min_texel_offset(&self) -> i32 {
        self.inner.limits.min_texel_offset
    }

    /// Get the maximum texel offset supported by the adapter
    pub fn max_texel_offset(&self) -> u32 {
        self.inner.limits.max_texel_offset
    }

    /// Get the minimum texel gather offset supported by the adapter
    pub fn min_texel_gather_offset(&self) -> i32 {
        self.inner.limits.min_texel_gather_offset
    }

    /// Get the maximum texel gather offset supported by the adapter
    pub fn max_texel_gather_offset(&self) -> u32 {
        self.inner.limits.max_texel_gather_offset
    }

    /// Get the minimum interpolation offset supported by the adapter
    pub fn min_interpolation_offset(&self) -> f32 {
        self.inner.limits.min_interpolation_offset
    }

    /// Get the maximum interpolation offset supported by the adapter
    pub fn max_interpolation_offset(&self) -> f32 {
        self.inner.limits.max_interpolation_offset
    }

    /// Get the sub-pixel interpolation offset bits supported by the adapter
    pub fn sub_pixel_interpolation_offset_bits(&self) -> u32 {
        self.inner.limits.sub_pixel_interpolation_offset_bits
    }

    /// Get the maximum framebuffer width supported by the adapter
    pub fn max_framebuffer_width(&self) -> u32 {
        self.inner.limits.max_framebuffer_width
    }

    /// Get the maximum framebuffer height supported by the adapter
    pub fn max_framebuffer_height(&self) -> u32 {
        self.inner.limits.max_framebuffer_height
    }

    /// Get the maximum framebuffer layers supported by the adapter
    pub fn max_framebuffer_layers(&self) -> u32 {
        self.inner.limits.max_framebuffer_layers
    }

    /// Get the framebuffer color sample counts supported by the adapter
    pub fn framebuffer_color_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.framebuffer_color_sample_counts
    }

    /// Get the framebuffer depth sample counts supported by the adapter
    pub fn framebuffer_depth_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.framebuffer_depth_sample_counts
    }

    /// Get the framebuffer stencil sample counts supported by the adapter
    pub fn framebuffer_stencil_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.framebuffer_stencil_sample_counts
    }

    /// Get the no-attachment framebuffer sample counts supported by the adapter
    pub fn framebuffer_no_attachments_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.framebuffer_no_attachments_sample_counts
    }

    /// Get the maximum number of color attachments supported by the adapter
    pub fn max_color_attachments(&self) -> u32 {
        self.inner.limits.max_color_attachments
    }

    /// Get the sampled image color sample counts supported by the adapter
    pub fn sampled_image_color_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.sampled_image_color_sample_counts
    }

    /// Get the sampled image integer sample counts supported by the adapter
    pub fn sampled_image_integer_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.sampled_image_integer_sample_counts
    }

    /// Get the sampled image depth sample counts supported by the adapter
    pub fn sampled_image_depth_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.sampled_image_depth_sample_counts
    }

    /// Get the sampled image stencil sample counts supported by the adapter
    pub fn sampled_image_stencil_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.sampled_image_stencil_sample_counts
    }

    /// Get the storage image sample counts supported by the adapter
    pub fn storage_image_sample_counts(&self) -> VulkanSampleCountFlags {
        self.inner.limits.storage_image_sample_counts
    }

    /// Get the maximum number of sample mask words supported by the adapter
    pub fn max_sample_mask_words(&self) -> u32 {
        self.inner.limits.max_sample_mask_words
    }

    /// Does the adapter support timestamps on graphics and compute queues?
    pub fn timestamp_compute_and_graphics(&self) -> bool {
        self.inner.limits.timestamp_compute_and_graphics != 0
    }

    /// Get the timestamp period of the adapter in nanoseconds
    pub fn timestamp_period(&self) -> f32 {
        self.inner.limits.timestamp_period
    }

    /// Get the maximum clip distances supported by the adapter
    pub fn max_clip_distances(&self) -> u32 {
        self.inner.limits.max_clip_distances
    }

    /// Get the maximum cull distances supported by the adapter
    pub fn max_cull_distances(&self) -> u32 {
        self.inner.limits.max_cull_distances
    }

    /// Get the maximum combined clip and cull distances supported by the adapter
    pub fn max_combined_clip_and_cull_distances(&self) -> u32 {
        self.inner.limits.max_combined_clip_and_cull_distances
    }

    /// Get the number of discrete queue priorities supported by the adapter
    pub fn discrete_queue_priorities(&self) -> u32 {
        self.inner.limits.discrete_queue_priorities
    }

    /// Get the point size range supported by the adapter
    pub fn point_size_range(&self) -> [f32; 2] {
        self.inner.limits.point_size_range
    }

    /// Get the line width range supported by the adapter
    pub fn line_width_range(&self) -> [f32; 2] {
        self.inner.limits.line_width_range
    }

    /// Get the point size granularity supported by the adapter
    pub fn point_size_granularity(&self) -> f32 {
        self.inner.limits.point_size_granularity
    }

    /// Get the line width granularity supported by the adapter
    pub fn line_width_granularity(&self) -> f32 {
        self.inner.limits.line_width_granularity
    }

    /// Does the adapter use strict line rasterization rules?
    pub fn strict_lines(&self) -> bool {
        self.inner.limits.strict_lines != 0
    }

    /// Does the adapter use standard sample locations?
    pub fn standard_sample_locations(&self) -> bool {
        self.inner.limits.standard_sample_locations != 0
    }

    /// Get the optimal buffer copy offset alignment supported by the adapter
    pub fn optimal_buffer_copy_offset_alignment(&self) -> u64 {
        self.inner.limits.optimal_buffer_copy_offset_alignment
    }

    /// Get the optimal buffer copy row pitch alignment supported by the adapter
    pub fn optimal_buffer_copy_row_pitch_alignment(&self) -> u64 {
        self.inner.limits.optimal_buffer_copy_row_pitch_alignment
    }

    /// Get the non-coherent atom size supported by the adapter
    pub fn non_coherent_atom_size(&self) -> u64 {
        self.inner.limits.non_coherent_atom_size
    }

    /// Get the maximum image dimension 1D supported by the adapter
    pub fn max_image_dimension_1d(&self) -> u32 {
        self.inner.limits.max_image_dimension_1d
    }

    /// Get the maximum image dimension 2D supported by the adapter
    pub fn max_image_dimension_2d(&self) -> u32 {
        self.inner.limits.max_image_dimension_2d
    }

    /// Get the maximum image dimension 3D supported by the adapter
    pub fn max_image_dimension_3d(&self) -> u32 {
        self.inner.limits.max_image_dimension_3d
    }

    /// Get the maximum cube image dimension supported by the adapter
    pub fn max_image_dimension_cube(&self) -> u32 {
        self.inner.limits.max_image_dimension_cube
    }

    /// Get the maximum image array layers supported by the adapter
    pub fn max_image_array_layers(&self) -> u32 {
        self.inner.limits.max_image_array_layers
    }

    /// Get the maximum texel buffer elements supported by the adapter
    pub fn max_texel_buffer_elements(&self) -> u32 {
        self.inner.limits.max_texel_buffer_elements
    }

    /// Get the maximum uniform buffer range supported by the adapter
    pub fn max_uniform_buffer_range(&self) -> u32 {
        self.inner.limits.max_uniform_buffer_range
    }

    /// Get the maximum storage buffer range supported by the adapter
    pub fn max_storage_buffer_range(&self) -> u32 {
        self.inner.limits.max_storage_buffer_range
    }

    /// Get the sparse address space size supported by the adapter
    pub fn sparse_address_space_size(&self) -> u64 {
        self.inner.limits.sparse_address_space_size
    }
}
