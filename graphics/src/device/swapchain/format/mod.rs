use other::OtherSwapchainFormat;

mod other;

mod from_vk;
mod into_vk;

/// The format swapchain images can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapchainFormat {
    /// A four-component, 32-bit unsigned normalized format that has an 8-bit B component in byte
    /// 0, an 8-bit G component in byte 1, an 8-bit R component in byte 2, and an 8-bit A component
    /// in byte 3.
    ///
    /// The values will be interpreted directly as SRGB values, without correction.
    B8G8R8A8Srgb,

    /// A four-component, 32-bit unsigned normalized format that has an 8-bit R component in byte
    /// 0, an 8-bit G component in byte 1, an 8-bit B component in byte 2, and an 8-bit A component
    /// in byte 3.
    ///
    /// The values will be interpreted directly as SRGB values, without correction.
    R8G8B8A8Srgb,

    /// A non-standard unsigned normalized format
    ///
    /// The values will be interpreted directly as SRGB values, without correction.
    OtherSrgb(OtherSwapchainFormat),
}
