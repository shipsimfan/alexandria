use crate::GraphicsVersion;
use vulkan::{VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3, VK_VERSION_1_4};

impl GraphicsVersion {
    /// Version 1.0 of Vulkan
    pub const VERSION_1_0: GraphicsVersion = unsafe { GraphicsVersion::new_raw(VK_VERSION_1_0) };

    /// Version 1.1 of Vulkan
    pub const VERSION_1_1: GraphicsVersion = unsafe { GraphicsVersion::new_raw(VK_VERSION_1_1) };

    /// Version 1.2 of Vulkan
    pub const VERSION_1_2: GraphicsVersion = unsafe { GraphicsVersion::new_raw(VK_VERSION_1_2) };

    /// Version 1.3 of Vulkan
    pub const VERSION_1_3: GraphicsVersion = unsafe { GraphicsVersion::new_raw(VK_VERSION_1_3) };

    /// Version 1.4 of Vulkan
    pub const VERSION_1_4: GraphicsVersion = unsafe { GraphicsVersion::new_raw(VK_VERSION_1_4) };
}
