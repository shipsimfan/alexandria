use crate::GraphicsDeviceExtension;

macro_rules! layer_cstrs {
    [$(
        $(#[$meta: meta])*
        $variant: ident => $str: literal,
    )*] => {
        impl GraphicsDeviceExtension {
            /// Try to convert `str` into a [`GraphicsDeviceExtension`]
            pub(in crate::device) fn from_str(str: &str) -> Option<GraphicsDeviceExtension> {
                match str {
                    $(
                        $(#[$meta])*
                        $str => Some(GraphicsDeviceExtension::$variant),
                    )*
                    _ => None,
                }
            }

            /// Get the Vulkan string representation of this layer
            pub(in crate::device) fn as_cstr(&self) -> &'static std::ffi::CStr {
                match self {$(
                    $(#[$meta])*
                    GraphicsDeviceExtension::$variant => unsafe {
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
];
