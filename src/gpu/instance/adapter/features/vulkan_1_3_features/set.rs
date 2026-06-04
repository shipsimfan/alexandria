use crate::gpu::VulkanDeviceVulkan13Features;
use vulkan::{VK_FALSE, VK_TRUE};

// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanCommandBuffer;

impl VulkanDeviceVulkan13Features {
    /// Enable robust image access guarantees for shader image accesses
    pub fn enable_robust_image_access(mut self) -> Self {
        self.inner.robust_image_access = VK_TRUE;
        self
    }

    /// Disable robust image access guarantees for shader image accesses
    pub fn disable_robust_image_access(mut self) -> Self {
        self.inner.robust_image_access = VK_FALSE;
        self
    }

    /// Enable inline uniform block descriptors
    pub fn enable_inline_uniform_block(mut self) -> Self {
        self.inner.inline_uniform_block = VK_TRUE;
        self
    }

    /// Disable inline uniform block descriptors
    pub fn disable_inline_uniform_block(mut self) -> Self {
        self.inner.inline_uniform_block = VK_FALSE;
        self
    }

    /// Enable updating inline uniform block descriptors after a set is bound
    pub fn enable_descriptor_binding_inline_uniform_block_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_inline_uniform_block_update_after_bind = VK_TRUE;
        self
    }

    /// Disable updating inline uniform block descriptors after a set is bound
    pub fn disable_descriptor_binding_inline_uniform_block_update_after_bind(mut self) -> Self {
        self.inner
            .descriptor_binding_inline_uniform_block_update_after_bind = VK_FALSE;
        self
    }

    /// Enable support for pipeline creation cache control
    pub fn enable_pipeline_creation_cache_control(mut self) -> Self {
        self.inner.pipeline_creation_cache_control = VK_TRUE;
        self
    }

    /// Disable support for pipeline creation cache control
    pub fn disable_pipeline_creation_cache_control(mut self) -> Self {
        self.inner.pipeline_creation_cache_control = VK_FALSE;
        self
    }

    /// Enable support for private data
    pub fn enable_private_data(mut self) -> Self {
        self.inner.private_data = VK_TRUE;
        self
    }

    /// Disable support for private data
    pub fn disable_private_data(mut self) -> Self {
        self.inner.private_data = VK_FALSE;
        self
    }

    /// Enable support for shader demote to helper invocation
    pub fn enable_shader_demote_to_helper_invocation(mut self) -> Self {
        self.inner.shader_demote_to_helper_invocation = VK_TRUE;
        self
    }

    /// Disable support for shader demote to helper invocation
    pub fn disable_shader_demote_to_helper_invocation(mut self) -> Self {
        self.inner.shader_demote_to_helper_invocation = VK_FALSE;
        self
    }

    /// Enable support for shader terminate invocation
    pub fn enable_shader_terminate_invocation(mut self) -> Self {
        self.inner.shader_terminate_invocation = VK_TRUE;
        self
    }

    /// Disable support for shader terminate invocation
    pub fn disable_shader_terminate_invocation(mut self) -> Self {
        self.inner.shader_terminate_invocation = VK_FALSE;
        self
    }

    /// Enable support for subgroup size control
    pub fn enable_subgroup_size_control(mut self) -> Self {
        self.inner.subgroup_size_control = VK_TRUE;
        self
    }

    /// Disable support for subgroup size control
    pub fn disable_subgroup_size_control(mut self) -> Self {
        self.inner.subgroup_size_control = VK_FALSE;
        self
    }

    /// Enable support for compute full subgroups
    pub fn enable_compute_full_subgroups(mut self) -> Self {
        self.inner.compute_full_subgroups = VK_TRUE;
        self
    }

    /// Disable support for compute full subgroups
    pub fn disable_compute_full_subgroups(mut self) -> Self {
        self.inner.compute_full_subgroups = VK_FALSE;
        self
    }

    /// Enable the new set of synchronization commands introduced in `khr_synchronization2`.
    pub fn enable_synchronization2(mut self) -> Self {
        self.inner.synchronization2 = VK_TRUE;
        self
    }

    /// Disable the new set of synchronization commands introduced in `khr_synchronization2`.
    pub fn disable_synchronization2(mut self) -> Self {
        self.inner.synchronization2 = VK_FALSE;
        self
    }

    /// Enable support for ASTC HDR texture compression
    pub fn enable_texture_compression_astc_hdr(mut self) -> Self {
        self.inner.texture_compression_astc_hdr = VK_TRUE;
        self
    }

    /// Disable support for ASTC HDR texture compression
    pub fn disable_texture_compression_astc_hdr(mut self) -> Self {
        self.inner.texture_compression_astc_hdr = VK_FALSE;
        self
    }

    /// Enable support for shader zero initialize workgroup memory
    pub fn enable_shader_zero_initialize_workgroup_memory(mut self) -> Self {
        self.inner.shader_zero_initialize_workgroup_memory = VK_TRUE;
        self
    }

    /// Disable support for shader zero initialize workgroup memory
    pub fn disable_shader_zero_initialize_workgroup_memory(mut self) -> Self {
        self.inner.shader_zero_initialize_workgroup_memory = VK_FALSE;
        self
    }

    /// Enable dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command.
    pub fn enable_dynamic_rendering(mut self) -> Self {
        self.inner.dynamic_rendering = VK_TRUE;
        self
    }

    /// Disable dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command.
    pub fn disable_dynamic_rendering(mut self) -> Self {
        self.inner.dynamic_rendering = VK_FALSE;
        self
    }

    /// Enable support for integer dot product operations in shaders
    pub fn enable_shader_integer_dot_product(mut self) -> Self {
        self.inner.shader_integer_dot_product = VK_TRUE;
        self
    }

    /// Disable support for integer dot product operations in shaders
    pub fn disable_shader_integer_dot_product(mut self) -> Self {
        self.inner.shader_integer_dot_product = VK_FALSE;
        self
    }

    /// Enable the `maintenance4` features
    pub fn enable_maintenance4(mut self) -> Self {
        self.inner.maintenance4 = VK_TRUE;
        self
    }

    /// Disable the `maintenance4` features
    pub fn disable_maintenance4(mut self) -> Self {
        self.inner.maintenance4 = VK_FALSE;
        self
    }
}
