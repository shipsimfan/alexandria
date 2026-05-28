// rustdoc imports
#[allow(unused_imports)]
use crate::gpu::VulkanVersion;

/// Create a [`VulkanVersion`] from the `CARGO_PKG_VERSION` environment variables
#[macro_export]
macro_rules! cargo_vulkan_version {
    () => {
        $crate::gpu::VulkanVersion::from_strs(
            None,
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
        )
        .expect("invalid CARGO_PKG_VERSION environment variables")
    };
}
