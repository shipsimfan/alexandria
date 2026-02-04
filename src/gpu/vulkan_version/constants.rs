use crate::gpu::VulkanVersion;
use vulkan::{VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3, VK_VERSION_1_4};

impl VulkanVersion {
    /// Version 1.0 of Vulkan
    pub const VERSION_1_0: VulkanVersion = unsafe { VulkanVersion::new_raw(VK_VERSION_1_0) };

    /// Version 1.1 of Vulkan
    pub const VERSION_1_1: VulkanVersion = unsafe { VulkanVersion::new_raw(VK_VERSION_1_1) };

    /// Version 1.2 of Vulkan
    pub const VERSION_1_2: VulkanVersion = unsafe { VulkanVersion::new_raw(VK_VERSION_1_2) };

    /// Version 1.3 of Vulkan
    pub const VERSION_1_3: VulkanVersion = unsafe { VulkanVersion::new_raw(VK_VERSION_1_3) };

    /// Version 1.4 of Vulkan
    pub const VERSION_1_4: VulkanVersion = unsafe { VulkanVersion::new_raw(VK_VERSION_1_4) };
}
