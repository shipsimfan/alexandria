use crate::gpu::VulkanDeviceVulkan13Features;
use vulkan::VK_TRUE;

// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanCommandBuffer;

impl VulkanDeviceVulkan13Features {
    /// Get whether robust image access guarantees for shader image accesses is enabled
    pub fn robust_image_access(&self) -> bool {
        self.inner.robust_image_access == VK_TRUE
    }

    /// Get whether inline uniform block descriptors is enabled
    pub fn inline_uniform_block(&self) -> bool {
        self.inner.inline_uniform_block == VK_TRUE
    }

    /// Get whether updating inline uniform block descriptors after a set is bound is enabled
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(&self) -> bool {
        self.inner
            .descriptor_binding_inline_uniform_block_update_after_bind
            == VK_TRUE
    }

    /// Get whether support for pipeline creation cache control is enabled
    pub fn pipeline_creation_cache_control(&self) -> bool {
        self.inner.pipeline_creation_cache_control == VK_TRUE
    }

    /// Get whether support for private data is enabled
    pub fn private_data(&self) -> bool {
        self.inner.private_data == VK_TRUE
    }

    /// Get whether support for demoting shader invocations to helper invocations is enabled
    pub fn shader_demote_to_helper_invocation(&self) -> bool {
        self.inner.shader_demote_to_helper_invocation == VK_TRUE
    }

    /// Get whether support for shader termination is enabled
    pub fn shader_terminate_invocation(&self) -> bool {
        self.inner.shader_terminate_invocation == VK_TRUE
    }

    /// Get whether support for subgroup size control is enabled
    pub fn subgroup_size_control(&self) -> bool {
        self.inner.subgroup_size_control == VK_TRUE
    }

    /// Get whether support for compute full subgroups is enabled
    pub fn compute_full_subgroups(&self) -> bool {
        self.inner.compute_full_subgroups == VK_TRUE
    }

    /// Get whether the new set of synchronization commands introduced in `khr_synchronization2` is enabled
    pub fn synchronization2(&self) -> bool {
        self.inner.synchronization2 == VK_TRUE
    }

    /// Get whether support for all of the ASTC HDR compressed texture formats is enabled
    pub fn texture_compression_astc_hdr(&self) -> bool {
        self.inner.texture_compression_astc_hdr == VK_TRUE
    }

    /// Get whether support for zero initialize workgroup memory is enabled
    pub fn shader_zero_initialize_workgroup_memory(&self) -> bool {
        self.inner.shader_zero_initialize_workgroup_memory == VK_TRUE
    }

    /// Get whether dynamic render pass instances using the [`VulkanCommandBuffer::cmd_begin_rendering`]
    /// command is enabled
    pub fn dynamic_rendering(&self) -> bool {
        self.inner.dynamic_rendering == VK_TRUE
    }

    /// Get whether integer dot products in shaders is enabled
    pub fn shader_integer_dot_product(&self) -> bool {
        self.inner.shader_integer_dot_product == VK_TRUE
    }

    /// Get whether support for maintenance4 is enabled
    pub fn maintenance4(&self) -> bool {
        self.inner.maintenance4 == VK_TRUE
    }
}
