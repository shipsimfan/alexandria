use crate::GraphicsInstanceExtension;

macro_rules! layer_cstrs {
    [$(
        $(#[$meta: meta])*
        $variant: ident => $str: literal,
    )*] => {
        impl GraphicsInstanceExtension {
            /// Try to convert `str` into a [`GraphicsInstanceExtension`]
            pub(in crate::instance) fn from_str(str: &str) -> Option<GraphicsInstanceExtension> {
                match str {
                    $(
                        $(#[$meta])*
                        $str => Some(GraphicsInstanceExtension::$variant),
                    )*
                    _ => None,
                }
            }

            /// Get the Vulkan string representation of this layer
            pub(in crate::instance) fn as_cstr(&self) -> &'static std::ffi::CStr {
                match self {$(
                    $(#[$meta])*
                    GraphicsInstanceExtension::$variant => unsafe {
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
    DebugUtils => "VK_EXT_debug_utils",
    Surface => "VK_KHR_surface",
    #[cfg(target_os = "windows")]
    Win32Surface => "VK_KHR_win32_surface",
    #[cfg(target_os = "linux")]
    WaylandSurface => "VK_KHR_wayland_surface",
];
