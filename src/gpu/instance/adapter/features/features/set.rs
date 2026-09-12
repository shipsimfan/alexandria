use crate::gpu::VulkanDeviceFeatures;
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanDeviceFeatures {
    /// Enable support for robust buffer access
    pub fn enable_robust_buffer_access(mut self) -> Self {
        self.inner.features.robust_buffer_access = VK_TRUE;
        self
    }

    /// Disable support for robust buffer access
    pub fn disable_robust_buffer_access(mut self) -> Self {
        self.inner.features.robust_buffer_access = VK_FALSE;
        self
    }

    /// Enable support for the full 32-bit index range
    pub fn enable_full_draw_index_uint32(mut self) -> Self {
        self.inner.features.full_draw_index_uint32 = VK_TRUE;
        self
    }

    /// Disable support for the full 32-bit index range
    pub fn disable_full_draw_index_uint32(mut self) -> Self {
        self.inner.features.full_draw_index_uint32 = VK_FALSE;
        self
    }

    /// Enable support for cube array images
    pub fn enable_image_cube_array(mut self) -> Self {
        self.inner.features.image_cube_array = VK_TRUE;
        self
    }

    /// Disable support for cube array images
    pub fn disable_image_cube_array(mut self) -> Self {
        self.inner.features.image_cube_array = VK_FALSE;
        self
    }

    /// Enable independent blending
    pub fn enable_independent_blend(mut self) -> Self {
        self.inner.features.independent_blend = VK_TRUE;
        self
    }

    /// Disable independent blending
    pub fn disable_independent_blend(mut self) -> Self {
        self.inner.features.independent_blend = VK_FALSE;
        self
    }

    /// Enable support for geometry shaders
    pub fn enable_geometry_shader(mut self) -> Self {
        self.inner.features.geometry_shader = VK_TRUE;
        self
    }

    /// Disable support for geometry shaders
    pub fn disable_geometry_shader(mut self) -> Self {
        self.inner.features.geometry_shader = VK_FALSE;
        self
    }

    /// Enable support for tessellation shaders
    pub fn enable_tessellation_shader(mut self) -> Self {
        self.inner.features.tessellation_shader = VK_TRUE;
        self
    }

    /// Disable support for tessellation shaders
    pub fn disable_tessellation_shader(mut self) -> Self {
        self.inner.features.tessellation_shader = VK_FALSE;
        self
    }

    /// Enable support for sample rate shading
    pub fn enable_sample_rate_shading(mut self) -> Self {
        self.inner.features.sample_rate_shading = VK_TRUE;
        self
    }

    /// Disable support for sample rate shading
    pub fn disable_sample_rate_shading(mut self) -> Self {
        self.inner.features.sample_rate_shading = VK_FALSE;
        self
    }

    /// Enable support for dual-source blending
    pub fn enable_dual_src_blend(mut self) -> Self {
        self.inner.features.dual_src_blend = VK_TRUE;
        self
    }

    /// Disable support for dual-source blending
    pub fn disable_dual_src_blend(mut self) -> Self {
        self.inner.features.dual_src_blend = VK_FALSE;
        self
    }

    /// Enable support for logic operations
    pub fn enable_logic_op(mut self) -> Self {
        self.inner.features.logic_op = VK_TRUE;
        self
    }

    /// Disable support for logic operations
    pub fn disable_logic_op(mut self) -> Self {
        self.inner.features.logic_op = VK_FALSE;
        self
    }

    /// Enable support for multi-draw indirect
    pub fn enable_multi_draw_indirect(mut self) -> Self {
        self.inner.features.multi_draw_indirect = VK_TRUE;
        self
    }

    /// Disable support for multi-draw indirect
    pub fn disable_multi_draw_indirect(mut self) -> Self {
        self.inner.features.multi_draw_indirect = VK_FALSE;
        self
    }

    /// Enable support for an indirect draw first instance
    pub fn enable_draw_indirect_first_instance(mut self) -> Self {
        self.inner.features.draw_indirect_first_instance = VK_TRUE;
        self
    }

    /// Disable support for an indirect draw first instance
    pub fn disable_draw_indirect_first_instance(mut self) -> Self {
        self.inner.features.draw_indirect_first_instance = VK_FALSE;
        self
    }

    /// Enable support for depth clamping
    pub fn enable_depth_clamp(mut self) -> Self {
        self.inner.features.depth_clamp = VK_TRUE;
        self
    }

    /// Disable support for depth clamping
    pub fn disable_depth_clamp(mut self) -> Self {
        self.inner.features.depth_clamp = VK_FALSE;
        self
    }

    /// Enable support for depth bias clamping
    pub fn enable_depth_bias_clamp(mut self) -> Self {
        self.inner.features.depth_bias_clamp = VK_TRUE;
        self
    }

    /// Disable support for depth bias clamping
    pub fn disable_depth_bias_clamp(mut self) -> Self {
        self.inner.features.depth_bias_clamp = VK_FALSE;
        self
    }

    /// Enable support for non-solid fill modes
    pub fn enable_fill_mode_non_solid(mut self) -> Self {
        self.inner.features.fill_mode_non_solid = VK_TRUE;
        self
    }

    /// Disable support for non-solid fill modes
    pub fn disable_fill_mode_non_solid(mut self) -> Self {
        self.inner.features.fill_mode_non_solid = VK_FALSE;
        self
    }

    /// Enable support for depth bounds tests
    pub fn enable_depth_bounds(mut self) -> Self {
        self.inner.features.depth_bounds = VK_TRUE;
        self
    }

    /// Disable support for depth bounds tests
    pub fn disable_depth_bounds(mut self) -> Self {
        self.inner.features.depth_bounds = VK_FALSE;
        self
    }

    /// Enable support for wide lines
    pub fn enable_wide_lines(mut self) -> Self {
        self.inner.features.wide_lines = VK_TRUE;
        self
    }

    /// Disable support for wide lines
    pub fn disable_wide_lines(mut self) -> Self {
        self.inner.features.wide_lines = VK_FALSE;
        self
    }

    /// Enable support for large points
    pub fn enable_large_points(mut self) -> Self {
        self.inner.features.large_points = VK_TRUE;
        self
    }

    /// Disable support for large points
    pub fn disable_large_points(mut self) -> Self {
        self.inner.features.large_points = VK_FALSE;
        self
    }

    /// Enable alpha-to-one
    pub fn enable_alpha_to_one(mut self) -> Self {
        self.inner.features.alpha_to_one = VK_TRUE;
        self
    }

    /// Disable alpha-to-one
    pub fn disable_alpha_to_one(mut self) -> Self {
        self.inner.features.alpha_to_one = VK_FALSE;
        self
    }

    /// Enable support for multiple viewports
    pub fn enable_multi_viewport(mut self) -> Self {
        self.inner.features.multi_viewport = VK_TRUE;
        self
    }

    /// Disable support for multiple viewports
    pub fn disable_multi_viewport(mut self) -> Self {
        self.inner.features.multi_viewport = VK_FALSE;
        self
    }

    /// Enable support for sampler anisotropy
    pub fn enable_sampler_anisotropy(mut self) -> Self {
        self.inner.features.sampler_anisotropy = VK_TRUE;
        self
    }

    /// Disable support for sampler anisotropy
    pub fn disable_sampler_anisotropy(mut self) -> Self {
        self.inner.features.sampler_anisotropy = VK_FALSE;
        self
    }

    /// Enable support for ETC2 texture compression
    pub fn enable_texture_compression_etc2(mut self) -> Self {
        self.inner.features.texture_compression_etc2 = VK_TRUE;
        self
    }

    /// Disable support for ETC2 texture compression
    pub fn disable_texture_compression_etc2(mut self) -> Self {
        self.inner.features.texture_compression_etc2 = VK_FALSE;
        self
    }

    /// Enable support for ASTC LDR texture compression
    pub fn enable_texture_compression_astcldr(mut self) -> Self {
        self.inner.features.texture_compression_astcldr = VK_TRUE;
        self
    }

    /// Disable support for ASTC LDR texture compression
    pub fn disable_texture_compression_astcldr(mut self) -> Self {
        self.inner.features.texture_compression_astcldr = VK_FALSE;
        self
    }

    /// Enable support for BC texture compression
    pub fn enable_texture_compression_bc(mut self) -> Self {
        self.inner.features.texture_compression_bc = VK_TRUE;
        self
    }

    /// Disable support for BC texture compression
    pub fn disable_texture_compression_bc(mut self) -> Self {
        self.inner.features.texture_compression_bc = VK_FALSE;
        self
    }

    /// Enable support for precise occlusion queries
    pub fn enable_occlusion_query_precise(mut self) -> Self {
        self.inner.features.occlusion_query_precise = VK_TRUE;
        self
    }

    /// Disable support for precise occlusion queries
    pub fn disable_occlusion_query_precise(mut self) -> Self {
        self.inner.features.occlusion_query_precise = VK_FALSE;
        self
    }

    /// Enable support for pipeline statistics queries
    pub fn enable_pipeline_statistics_query(mut self) -> Self {
        self.inner.features.pipeline_statistics_query = VK_TRUE;
        self
    }

    /// Disable support for pipeline statistics queries
    pub fn disable_pipeline_statistics_query(mut self) -> Self {
        self.inner.features.pipeline_statistics_query = VK_FALSE;
        self
    }

    /// Enable support for vertex pipeline stores and atomics
    pub fn enable_vertex_pipeline_stores_and_atomics(mut self) -> Self {
        self.inner.features.vertex_pipeline_stores_and_atomics = VK_TRUE;
        self
    }

    /// Disable support for vertex pipeline stores and atomics
    pub fn disable_vertex_pipeline_stores_and_atomics(mut self) -> Self {
        self.inner.features.vertex_pipeline_stores_and_atomics = VK_FALSE;
        self
    }

    /// Enable support for fragment stores and atomics
    pub fn enable_fragment_stores_and_atomics(mut self) -> Self {
        self.inner.features.fragment_stores_and_atomics = VK_TRUE;
        self
    }

    /// Disable support for fragment stores and atomics
    pub fn disable_fragment_stores_and_atomics(mut self) -> Self {
        self.inner.features.fragment_stores_and_atomics = VK_FALSE;
        self
    }

    /// Enable support for tessellation and geometry shader point size
    pub fn enable_shader_tessellation_and_geometry_point_size(mut self) -> Self {
        self.inner
            .features
            .shader_tessellation_and_geometry_point_size = VK_TRUE;
        self
    }

    /// Disable support for tessellation and geometry shader point size
    pub fn disable_shader_tessellation_and_geometry_point_size(mut self) -> Self {
        self.inner
            .features
            .shader_tessellation_and_geometry_point_size = VK_FALSE;
        self
    }

    /// Enable support for extended shader image gather instructions
    pub fn enable_shader_image_gather_extended(mut self) -> Self {
        self.inner.features.shader_image_gather_extended = VK_TRUE;
        self
    }

    /// Disable support for extended shader image gather instructions
    pub fn disable_shader_image_gather_extended(mut self) -> Self {
        self.inner.features.shader_image_gather_extended = VK_FALSE;
        self
    }

    /// Enable support for extended shader storage image formats
    pub fn enable_shader_storage_image_extended_formats(mut self) -> Self {
        self.inner.features.shader_storage_image_extended_formats = VK_TRUE;
        self
    }

    /// Disable support for extended shader storage image formats
    pub fn disable_shader_storage_image_extended_formats(mut self) -> Self {
        self.inner.features.shader_storage_image_extended_formats = VK_FALSE;
        self
    }

    /// Enable support for multisampled shader storage images
    pub fn enable_shader_storage_image_multisample(mut self) -> Self {
        self.inner.features.shader_storage_image_multisample = VK_TRUE;
        self
    }

    /// Disable support for multisampled shader storage images
    pub fn disable_shader_storage_image_multisample(mut self) -> Self {
        self.inner.features.shader_storage_image_multisample = VK_FALSE;
        self
    }

    /// Enable support for storage image reads without a format
    pub fn enable_shader_storage_image_read_without_format(mut self) -> Self {
        self.inner.features.shader_storage_image_read_without_format = VK_TRUE;
        self
    }

    /// Disable support for storage image reads without a format
    pub fn disable_shader_storage_image_read_without_format(mut self) -> Self {
        self.inner.features.shader_storage_image_read_without_format = VK_FALSE;
        self
    }

    /// Enable support for storage image writes without a format
    pub fn enable_shader_storage_image_write_without_format(mut self) -> Self {
        self.inner
            .features
            .shader_storage_image_write_without_format = VK_TRUE;
        self
    }

    /// Disable support for storage image writes without a format
    pub fn disable_shader_storage_image_write_without_format(mut self) -> Self {
        self.inner
            .features
            .shader_storage_image_write_without_format = VK_FALSE;
        self
    }

    /// Enable dynamic indexing of uniform buffer arrays
    pub fn enable_shader_uniform_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_uniform_buffer_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable dynamic indexing of uniform buffer arrays
    pub fn disable_shader_uniform_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_uniform_buffer_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable dynamic indexing of sampled image arrays
    pub fn enable_shader_sampled_image_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_sampled_image_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable dynamic indexing of sampled image arrays
    pub fn disable_shader_sampled_image_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_sampled_image_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable dynamic indexing of storage buffer arrays
    pub fn enable_shader_storage_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_storage_buffer_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable dynamic indexing of storage buffer arrays
    pub fn disable_shader_storage_buffer_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_storage_buffer_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable dynamic indexing of storage image arrays
    pub fn enable_shader_storage_image_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_storage_image_array_dynamic_indexing = VK_TRUE;
        self
    }

    /// Disable dynamic indexing of storage image arrays
    pub fn disable_shader_storage_image_array_dynamic_indexing(mut self) -> Self {
        self.inner
            .features
            .shader_storage_image_array_dynamic_indexing = VK_FALSE;
        self
    }

    /// Enable support for shader clip distance
    pub fn enable_shader_clip_distance(mut self) -> Self {
        self.inner.features.shader_clip_distance = VK_TRUE;
        self
    }

    /// Disable support for shader clip distance
    pub fn disable_shader_clip_distance(mut self) -> Self {
        self.inner.features.shader_clip_distance = VK_FALSE;
        self
    }

    /// Enable support for shader cull distance
    pub fn enable_shader_cull_distance(mut self) -> Self {
        self.inner.features.shader_cull_distance = VK_TRUE;
        self
    }

    /// Disable support for shader cull distance
    pub fn disable_shader_cull_distance(mut self) -> Self {
        self.inner.features.shader_cull_distance = VK_FALSE;
        self
    }

    /// Enable support for 64-bit floating-point shader operations
    pub fn enable_shader_float64(mut self) -> Self {
        self.inner.features.shader_float64 = VK_TRUE;
        self
    }

    /// Disable support for 64-bit floating-point shader operations
    pub fn disable_shader_float64(mut self) -> Self {
        self.inner.features.shader_float64 = VK_FALSE;
        self
    }

    /// Enable support for 64-bit integer shader operations
    pub fn enable_shader_int64(mut self) -> Self {
        self.inner.features.shader_int64 = VK_TRUE;
        self
    }

    /// Disable support for 64-bit integer shader operations
    pub fn disable_shader_int64(mut self) -> Self {
        self.inner.features.shader_int64 = VK_FALSE;
        self
    }

    /// Enable support for 16-bit integer shader operations
    pub fn enable_shader_int16(mut self) -> Self {
        self.inner.features.shader_int16 = VK_TRUE;
        self
    }

    /// Disable support for 16-bit integer shader operations
    pub fn disable_shader_int16(mut self) -> Self {
        self.inner.features.shader_int16 = VK_FALSE;
        self
    }

    /// Enable support for shader resource residency
    pub fn enable_shader_resource_residency(mut self) -> Self {
        self.inner.features.shader_resource_residency = VK_TRUE;
        self
    }

    /// Disable support for shader resource residency
    pub fn disable_shader_resource_residency(mut self) -> Self {
        self.inner.features.shader_resource_residency = VK_FALSE;
        self
    }

    /// Enable support for the shader resource minimum LOD
    pub fn enable_shader_resource_min_lod(mut self) -> Self {
        self.inner.features.shader_resource_min_lod = VK_TRUE;
        self
    }

    /// Disable support for the shader resource minimum LOD
    pub fn disable_shader_resource_min_lod(mut self) -> Self {
        self.inner.features.shader_resource_min_lod = VK_FALSE;
        self
    }

    /// Enable support for sparse binding
    pub fn enable_sparse_binding(mut self) -> Self {
        self.inner.features.sparse_binding = VK_TRUE;
        self
    }

    /// Disable support for sparse binding
    pub fn disable_sparse_binding(mut self) -> Self {
        self.inner.features.sparse_binding = VK_FALSE;
        self
    }

    /// Enable support for sparse resident buffers
    pub fn enable_sparse_residency_buffer(mut self) -> Self {
        self.inner.features.sparse_residency_buffer = VK_TRUE;
        self
    }

    /// Disable support for sparse resident buffers
    pub fn disable_sparse_residency_buffer(mut self) -> Self {
        self.inner.features.sparse_residency_buffer = VK_FALSE;
        self
    }

    /// Enable support for sparse resident 2D images
    pub fn enable_sparse_residency_image_2d(mut self) -> Self {
        self.inner.features.sparse_residency_image_2d = VK_TRUE;
        self
    }

    /// Disable support for sparse resident 2D images
    pub fn disable_sparse_residency_image_2d(mut self) -> Self {
        self.inner.features.sparse_residency_image_2d = VK_FALSE;
        self
    }

    /// Enable support for sparse resident 3D images
    pub fn enable_sparse_residency_image_3d(mut self) -> Self {
        self.inner.features.sparse_residency_image_3d = VK_TRUE;
        self
    }

    /// Disable support for sparse resident 3D images
    pub fn disable_sparse_residency_image_3d(mut self) -> Self {
        self.inner.features.sparse_residency_image_3d = VK_FALSE;
        self
    }

    /// Enable support for sparse resident 2-sample images
    pub fn enable_sparse_residency_2_samples(mut self) -> Self {
        self.inner.features.sparse_residency_2_samples = VK_TRUE;
        self
    }

    /// Disable support for sparse resident 2-sample images
    pub fn disable_sparse_residency_2_samples(mut self) -> Self {
        self.inner.features.sparse_residency_2_samples = VK_FALSE;
        self
    }

    /// Enable support for sparse resident 4-sample images
    pub fn enable_sparse_residency_4_samples(mut self) -> Self {
        self.inner.features.sparse_residency_4_samples = VK_TRUE;
        self
    }

    /// Disable support for sparse resident 4-sample images
    pub fn disable_sparse_residency_4_samples(mut self) -> Self {
        self.inner.features.sparse_residency_4_samples = VK_FALSE;
        self
    }

    /// Enable support for sparse resident 8-sample images
    pub fn enable_sparse_residency_8_samples(mut self) -> Self {
        self.inner.features.sparse_residency_8_samples = VK_TRUE;
        self
    }

    /// Disable support for sparse resident 8-sample images
    pub fn disable_sparse_residency_8_samples(mut self) -> Self {
        self.inner.features.sparse_residency_8_samples = VK_FALSE;
        self
    }

    /// Enable support for sparse resident 16-sample images
    pub fn enable_sparse_residency_16_samples(mut self) -> Self {
        self.inner.features.sparse_residency_16_samples = VK_TRUE;
        self
    }

    /// Disable support for sparse resident 16-sample images
    pub fn disable_sparse_residency_16_samples(mut self) -> Self {
        self.inner.features.sparse_residency_16_samples = VK_FALSE;
        self
    }

    /// Enable support for sparse residency aliasing
    pub fn enable_sparse_residency_aliased(mut self) -> Self {
        self.inner.features.sparse_residency_aliased = VK_TRUE;
        self
    }

    /// Disable support for sparse residency aliasing
    pub fn disable_sparse_residency_aliased(mut self) -> Self {
        self.inner.features.sparse_residency_aliased = VK_FALSE;
        self
    }

    /// Enable support for variable multisample rate
    pub fn enable_variable_multisample_rate(mut self) -> Self {
        self.inner.features.variable_multisample_rate = VK_TRUE;
        self
    }

    /// Disable support for variable multisample rate
    pub fn disable_variable_multisample_rate(mut self) -> Self {
        self.inner.features.variable_multisample_rate = VK_FALSE;
        self
    }

    /// Enable support for inherited queries
    pub fn enable_inherited_queries(mut self) -> Self {
        self.inner.features.inherited_queries = VK_TRUE;
        self
    }

    /// Disable support for inherited queries
    pub fn disable_inherited_queries(mut self) -> Self {
        self.inner.features.inherited_queries = VK_FALSE;
        self
    }
}
