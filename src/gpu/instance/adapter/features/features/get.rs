use crate::gpu::VulkanDeviceFeatures;
use vulkan::VK_TRUE;

impl VulkanDeviceFeatures {
    /// Is robust buffer access supported by the adapter?
    pub fn robust_buffer_access(&self) -> bool {
        self.inner.features.robust_buffer_access == VK_TRUE
    }

    /// Is the full 32-bit index range supported by the adapter?
    pub fn full_draw_index_uint32(&self) -> bool {
        self.inner.features.full_draw_index_uint32 == VK_TRUE
    }

    /// Are cube array images supported by the adapter?
    pub fn image_cube_array(&self) -> bool {
        self.inner.features.image_cube_array == VK_TRUE
    }

    /// Is independent blending supported by the adapter?
    pub fn independent_blend(&self) -> bool {
        self.inner.features.independent_blend == VK_TRUE
    }

    /// Are geometry shaders supported by the adapter?
    pub fn geometry_shader(&self) -> bool {
        self.inner.features.geometry_shader == VK_TRUE
    }

    /// Are tessellation shaders supported by the adapter?
    pub fn tessellation_shader(&self) -> bool {
        self.inner.features.tessellation_shader == VK_TRUE
    }

    /// Is sample rate shading supported by the adapter?
    pub fn sample_rate_shading(&self) -> bool {
        self.inner.features.sample_rate_shading == VK_TRUE
    }

    /// Is dual-source blending supported by the adapter?
    pub fn dual_src_blend(&self) -> bool {
        self.inner.features.dual_src_blend == VK_TRUE
    }

    /// Are logic operations supported by the adapter?
    pub fn logic_op(&self) -> bool {
        self.inner.features.logic_op == VK_TRUE
    }

    /// Is multi-draw indirect supported by the adapter?
    pub fn multi_draw_indirect(&self) -> bool {
        self.inner.features.multi_draw_indirect == VK_TRUE
    }

    /// Is indirect drawing with a non-zero first instance supported by the adapter?
    pub fn draw_indirect_first_instance(&self) -> bool {
        self.inner.features.draw_indirect_first_instance == VK_TRUE
    }

    /// Is depth clamping supported by the adapter?
    pub fn depth_clamp(&self) -> bool {
        self.inner.features.depth_clamp == VK_TRUE
    }

    /// Is depth bias clamping supported by the adapter?
    pub fn depth_bias_clamp(&self) -> bool {
        self.inner.features.depth_bias_clamp == VK_TRUE
    }

    /// Are non-solid fill modes supported by the adapter?
    pub fn fill_mode_non_solid(&self) -> bool {
        self.inner.features.fill_mode_non_solid == VK_TRUE
    }

    /// Are depth bounds tests supported by the adapter?
    pub fn depth_bounds(&self) -> bool {
        self.inner.features.depth_bounds == VK_TRUE
    }

    /// Are wide lines supported by the adapter?
    pub fn wide_lines(&self) -> bool {
        self.inner.features.wide_lines == VK_TRUE
    }

    /// Are large points supported by the adapter?
    pub fn large_points(&self) -> bool {
        self.inner.features.large_points == VK_TRUE
    }

    /// Is alpha-to-one supported by the adapter?
    pub fn alpha_to_one(&self) -> bool {
        self.inner.features.alpha_to_one == VK_TRUE
    }

    /// Is multiple viewport support enabled by the adapter?
    pub fn multi_viewport(&self) -> bool {
        self.inner.features.multi_viewport == VK_TRUE
    }

    /// Is sampler anisotropy supported by the adapter?
    pub fn sampler_anisotropy(&self) -> bool {
        self.inner.features.sampler_anisotropy == VK_TRUE
    }

    /// Is ETC2 texture compression supported by the adapter?
    pub fn texture_compression_etc2(&self) -> bool {
        self.inner.features.texture_compression_etc2 == VK_TRUE
    }

    /// Is ASTC LDR texture compression supported by the adapter?
    pub fn texture_compression_astcldr(&self) -> bool {
        self.inner.features.texture_compression_astcldr == VK_TRUE
    }

    /// Is BC texture compression supported by the adapter?
    pub fn texture_compression_bc(&self) -> bool {
        self.inner.features.texture_compression_bc == VK_TRUE
    }

    /// Are precise occlusion queries supported by the adapter?
    pub fn occlusion_query_precise(&self) -> bool {
        self.inner.features.occlusion_query_precise == VK_TRUE
    }

    /// Are pipeline statistics queries supported by the adapter?
    pub fn pipeline_statistics_query(&self) -> bool {
        self.inner.features.pipeline_statistics_query == VK_TRUE
    }

    /// Are vertex pipeline stores and atomics supported by the adapter?
    pub fn vertex_pipeline_stores_and_atomics(&self) -> bool {
        self.inner.features.vertex_pipeline_stores_and_atomics == VK_TRUE
    }

    /// Are fragment stores and atomics supported by the adapter?
    pub fn fragment_stores_and_atomics(&self) -> bool {
        self.inner.features.fragment_stores_and_atomics == VK_TRUE
    }

    /// Is tessellation and geometry shader point size supported by the adapter?
    pub fn shader_tessellation_and_geometry_point_size(&self) -> bool {
        self.inner
            .features
            .shader_tessellation_and_geometry_point_size
            == VK_TRUE
    }

    /// Are extended shader image gather instructions supported by the adapter?
    pub fn shader_image_gather_extended(&self) -> bool {
        self.inner.features.shader_image_gather_extended == VK_TRUE
    }

    /// Are extended shader storage image formats supported by the adapter?
    pub fn shader_storage_image_extended_formats(&self) -> bool {
        self.inner.features.shader_storage_image_extended_formats == VK_TRUE
    }

    /// Are multisampled shader storage images supported by the adapter?
    pub fn shader_storage_image_multisample(&self) -> bool {
        self.inner.features.shader_storage_image_multisample == VK_TRUE
    }

    /// Are storage image reads without a format supported by the adapter?
    pub fn shader_storage_image_read_without_format(&self) -> bool {
        self.inner.features.shader_storage_image_read_without_format == VK_TRUE
    }

    /// Are storage image writes without a format supported by the adapter?
    pub fn shader_storage_image_write_without_format(&self) -> bool {
        self.inner
            .features
            .shader_storage_image_write_without_format
            == VK_TRUE
    }

    /// Is dynamic indexing of uniform buffer arrays supported by the adapter?
    pub fn shader_uniform_buffer_array_dynamic_indexing(&self) -> bool {
        self.inner
            .features
            .shader_uniform_buffer_array_dynamic_indexing
            == VK_TRUE
    }

    /// Is dynamic indexing of sampled image arrays supported by the adapter?
    pub fn shader_sampled_image_array_dynamic_indexing(&self) -> bool {
        self.inner
            .features
            .shader_sampled_image_array_dynamic_indexing
            == VK_TRUE
    }

    /// Is dynamic indexing of storage buffer arrays supported by the adapter?
    pub fn shader_storage_buffer_array_dynamic_indexing(&self) -> bool {
        self.inner
            .features
            .shader_storage_buffer_array_dynamic_indexing
            == VK_TRUE
    }

    /// Is dynamic indexing of storage image arrays supported by the adapter?
    pub fn shader_storage_image_array_dynamic_indexing(&self) -> bool {
        self.inner
            .features
            .shader_storage_image_array_dynamic_indexing
            == VK_TRUE
    }

    /// Is shader clip distance supported by the adapter?
    pub fn shader_clip_distance(&self) -> bool {
        self.inner.features.shader_clip_distance == VK_TRUE
    }

    /// Is shader cull distance supported by the adapter?
    pub fn shader_cull_distance(&self) -> bool {
        self.inner.features.shader_cull_distance == VK_TRUE
    }

    /// Are 64-bit floating-point shader operations supported by the adapter?
    pub fn shader_float64(&self) -> bool {
        self.inner.features.shader_float64 == VK_TRUE
    }

    /// Are 64-bit integer shader operations supported by the adapter?
    pub fn shader_int64(&self) -> bool {
        self.inner.features.shader_int64 == VK_TRUE
    }

    /// Are 16-bit integer shader operations supported by the adapter?
    pub fn shader_int16(&self) -> bool {
        self.inner.features.shader_int16 == VK_TRUE
    }

    /// Is shader resource residency supported by the adapter?
    pub fn shader_resource_residency(&self) -> bool {
        self.inner.features.shader_resource_residency == VK_TRUE
    }

    /// Is the shader resource minimum LOD supported by the adapter?
    pub fn shader_resource_min_lod(&self) -> bool {
        self.inner.features.shader_resource_min_lod == VK_TRUE
    }

    /// Is sparse binding supported by the adapter?
    pub fn sparse_binding(&self) -> bool {
        self.inner.features.sparse_binding == VK_TRUE
    }

    /// Are sparse resident buffers supported by the adapter?
    pub fn sparse_residency_buffer(&self) -> bool {
        self.inner.features.sparse_residency_buffer == VK_TRUE
    }

    /// Are sparse resident 2D images supported by the adapter?
    pub fn sparse_residency_image_2d(&self) -> bool {
        self.inner.features.sparse_residency_image_2d == VK_TRUE
    }

    /// Are sparse resident 3D images supported by the adapter?
    pub fn sparse_residency_image_3d(&self) -> bool {
        self.inner.features.sparse_residency_image_3d == VK_TRUE
    }

    /// Are sparse resident 2-sample images supported by the adapter?
    pub fn sparse_residency_2_samples(&self) -> bool {
        self.inner.features.sparse_residency_2_samples == VK_TRUE
    }

    /// Are sparse resident 4-sample images supported by the adapter?
    pub fn sparse_residency_4_samples(&self) -> bool {
        self.inner.features.sparse_residency_4_samples == VK_TRUE
    }

    /// Are sparse resident 8-sample images supported by the adapter?
    pub fn sparse_residency_8_samples(&self) -> bool {
        self.inner.features.sparse_residency_8_samples == VK_TRUE
    }

    /// Are sparse resident 16-sample images supported by the adapter?
    pub fn sparse_residency_16_samples(&self) -> bool {
        self.inner.features.sparse_residency_16_samples == VK_TRUE
    }

    /// Is sparse residency aliasing supported by the adapter?
    pub fn sparse_residency_aliased(&self) -> bool {
        self.inner.features.sparse_residency_aliased == VK_TRUE
    }

    /// Is variable multisample rate supported by the adapter?
    pub fn variable_multisample_rate(&self) -> bool {
        self.inner.features.variable_multisample_rate == VK_TRUE
    }

    /// Are inherited queries supported by the adapter?
    pub fn inherited_queries(&self) -> bool {
        self.inner.features.inherited_queries == VK_TRUE
    }
}
