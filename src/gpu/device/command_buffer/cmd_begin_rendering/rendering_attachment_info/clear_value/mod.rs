mod from;
mod to_vk;

/// A clear value used for clearing attachments in dynamic rendering
pub enum VulkanClearValue {
    /// A clear value for floating-point color attachments
    ColorF32([f32; 4]),

    /// A clear value for signed integer color attachments
    ColorI32([i32; 4]),

    /// A clear value for unsigned integer color attachments
    ColorU32([u32; 4]),

    /// A clear value for depth-stencil attachments
    DepthStencil {
        /// The depth component of the clear value
        depth: f32,

        /// The stencil component of the clear value
        stencil: u32,
    },
}
