use crate::gpu::VulkanDeviceVulkan11Features;
use vulkan::VK_TRUE;

impl VulkanDeviceVulkan11Features {
    /// Get whether support for 16-bit access to storage buffers is enabled
    pub fn storage_buffer_16_bit_access(&self) -> bool {
        self.inner.storage_buffer_16_bit_access == VK_TRUE
    }

    /// Get whether support for 16-bit access to uniform and storage buffers is enabled
    pub fn uniform_and_storage_buffer_16_bit_access(&self) -> bool {
        self.inner.uniform_and_storage_buffer_16_bit_access == VK_TRUE
    }

    /// Get whether push constants support 16-bit types
    pub fn storage_push_constant_16(&self) -> bool {
        self.inner.storage_push_constant_16 == VK_TRUE
    }

    /// Get whether support for 16-bit access to input and output variables is enabled
    pub fn storage_input_output_16(&self) -> bool {
        self.inner.storage_input_output_16 == VK_TRUE
    }

    /// Get whether support for multiview is enabled
    pub fn multiview(&self) -> bool {
        self.inner.multiview == VK_TRUE
    }

    /// Get whether support for multiview geometry shader is enabled
    pub fn multiview_geometry_shader(&self) -> bool {
        self.inner.multiview_geometry_shader == VK_TRUE
    }

    /// Get whether support for multiview tessellation shader is enabled
    pub fn multiview_tessellation_shader(&self) -> bool {
        self.inner.multiview_tessellation_shader == VK_TRUE
    }

    /// Get whether support for variable pointers is enabled
    pub fn variable_pointers_storage_buffer(&self) -> bool {
        self.inner.variable_pointers_storage_buffer == VK_TRUE
    }

    /// Get whether support for variable pointers is enabled
    pub fn variable_pointers(&self) -> bool {
        self.inner.variable_pointers == VK_TRUE
    }

    /// Get whether support for protected memory is enabled
    pub fn protected_memory(&self) -> bool {
        self.inner.protected_memory == VK_TRUE
    }

    /// Get whether support for sampler YCbCr conversion is enabled
    pub fn sampler_ycbcr_conversion(&self) -> bool {
        self.inner.sampler_ycbcr_conversion == VK_TRUE
    }

    /// Get whether support for shader draw parameters is enabled
    pub fn shader_draw_parameters(&self) -> bool {
        self.inner.shader_draw_parameters == VK_TRUE
    }
}
