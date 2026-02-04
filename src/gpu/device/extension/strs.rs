use crate::gpu::VulkanDeviceExtension;

macro_rules! layer_cstrs {
    [$(
        $(#[$meta: meta])*
        $variant: ident => $str: literal,
    )*] => {
        impl VulkanDeviceExtension {
            /// Try to convert `str` into a [`VulkanDeviceExtension`]
            pub(crate) fn from_str(str: &str) -> Option<VulkanDeviceExtension> {
                match str {
                    $(
                        $(#[$meta])*
                        $str => Some(VulkanDeviceExtension::$variant),
                    )*
                    _ => None,
                }
            }

            /// Get the Vulkan string representation of this layer
            pub(in crate::gpu::device) fn as_cstr(&self) -> &'static std::ffi::CStr {
                match self {$(
                    $(#[$meta])*
                    VulkanDeviceExtension::$variant => unsafe {
                        std::ffi::CStr::from_bytes_with_nul_unchecked(
                            std::concat!($str, "\0").as_bytes()
                        )
                    },
                )*}
            }
        }
    };
}

layer_cstrs![
    Swapchain => "VK_KHR_swapchain",
    ExtendedDynamicState => "VK_EXT_extended_dynamic_state",
];
