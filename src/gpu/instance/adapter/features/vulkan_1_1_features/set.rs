use crate::gpu::VulkanDeviceVulkan11Features;
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanDeviceVulkan11Features {
    /// Enable support for 16-bit access to storage buffers
    pub fn enable_storage_buffer_16_bit_access(mut self) -> Self {
        self.inner.storage_buffer_16_bit_access = VK_TRUE;
        self
    }

    /// Disable support for 16-bit access to storage buffers
    pub fn disable_storage_buffer_16_bit_access(mut self) -> Self {
        self.inner.storage_buffer_16_bit_access = VK_FALSE;
        self
    }

    /// Enable support for 16-bit access to uniform and storage buffers
    pub fn enable_uniform_and_storage_buffer_16_bit_access(mut self) -> Self {
        self.inner.uniform_and_storage_buffer_16_bit_access = VK_TRUE;
        self
    }

    /// Disable support for 16-bit access to uniform and storage buffers
    pub fn disable_uniform_and_storage_buffer_16_bit_access(mut self) -> Self {
        self.inner.uniform_and_storage_buffer_16_bit_access = VK_FALSE;
        self
    }

    /// Enable push constants support 16-bit types
    pub fn enable_storage_push_constant_16(mut self) -> Self {
        self.inner.storage_push_constant_16 = VK_TRUE;
        self
    }

    /// Disable push constants support 16-bit types
    pub fn disable_storage_push_constant_16(mut self) -> Self {
        self.inner.storage_push_constant_16 = VK_FALSE;
        self
    }

    /// Enable support for 16-bit access to input and output variables
    pub fn enable_storage_input_output_16(mut self) -> Self {
        self.inner.storage_input_output_16 = VK_TRUE;
        self
    }

    /// Disable support for 16-bit access to input and output variables
    pub fn disable_storage_input_output_16(mut self) -> Self {
        self.inner.storage_input_output_16 = VK_FALSE;
        self
    }

    /// Enable support for multiview
    pub fn enable_multiview(mut self) -> Self {
        self.inner.multiview = VK_TRUE;
        self
    }

    /// Disable support for multiview
    pub fn disable_multiview(mut self) -> Self {
        self.inner.multiview = VK_FALSE;
        self
    }

    /// Enable support for multiview geometry shader
    pub fn enable_multiview_geometry_shader(mut self) -> Self {
        self.inner.multiview_geometry_shader = VK_TRUE;
        self
    }

    /// Disable support for multiview geometry shader
    pub fn disable_multiview_geometry_shader(mut self) -> Self {
        self.inner.multiview_geometry_shader = VK_FALSE;
        self
    }

    /// Enable support for multiview tessellation shader
    pub fn enable_multiview_tessellation_shader(mut self) -> Self {
        self.inner.multiview_tessellation_shader = VK_TRUE;
        self
    }

    /// Disable support for multiview tessellation shader
    pub fn disable_multiview_tessellation_shader(mut self) -> Self {
        self.inner.multiview_tessellation_shader = VK_FALSE;
        self
    }

    /// Enable support for variable pointers
    pub fn enable_variable_pointers_storage_buffer(mut self) -> Self {
        self.inner.variable_pointers_storage_buffer = VK_TRUE;
        self
    }

    /// Disable support for variable pointers
    pub fn disable_variable_pointers_storage_buffer(mut self) -> Self {
        self.inner.variable_pointers_storage_buffer = VK_FALSE;
        self
    }

    /// Enable support for variable pointers
    pub fn enable_variable_pointers(mut self) -> Self {
        self.inner.variable_pointers = VK_TRUE;
        self
    }

    /// Disable support for variable pointers
    pub fn disable_variable_pointers(mut self) -> Self {
        self.inner.variable_pointers = VK_FALSE;
        self
    }

    /// Enable support for protected memory
    pub fn enable_protected_memory(mut self) -> Self {
        self.inner.protected_memory = VK_TRUE;
        self
    }

    /// Disable support for protected memory
    pub fn disable_protected_memory(mut self) -> Self {
        self.inner.protected_memory = VK_FALSE;
        self
    }

    /// Enable support for sampler YCbCr conversion
    pub fn enable_sampler_ycbcr_conversion(mut self) -> Self {
        self.inner.sampler_ycbcr_conversion = VK_TRUE;
        self
    }

    /// Disable support for sampler YCbCr conversion
    pub fn disable_sampler_ycbcr_conversion(mut self) -> Self {
        self.inner.sampler_ycbcr_conversion = VK_FALSE;
        self
    }

    /// Enable support for shader draw parameters
    pub fn enable_shader_draw_parameters(mut self) -> Self {
        self.inner.shader_draw_parameters = VK_TRUE;
        self
    }

    /// Disable support for shader draw parameters
    pub fn disable_shader_draw_parameters(mut self) -> Self {
        self.inner.shader_draw_parameters = VK_FALSE;
        self
    }
}
