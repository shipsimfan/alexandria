use crate::GraphicsInstanceLayer;

macro_rules! layer_cstrs {
    [$($variant: ident => $str: literal,)*] => {
        impl GraphicsInstanceLayer {
            /// Try to convert `str` into a [`GraphicsInstanceLayer`]
            pub(in crate::instance) fn from_str(str: &str) -> Option<GraphicsInstanceLayer> {
                match str {
                    $(
                        $str => Some(GraphicsInstanceLayer::$variant),
                    )*
                    _ => None,
                }
            }

            /// Get the Vulkan string representation of this layer
            pub(in crate::instance) fn as_cstr(&self) -> &'static std::ffi::CStr {
                match self {$(
                    GraphicsInstanceLayer::$variant => unsafe {
                        std::ffi::CStr::from_bytes_with_nul_unchecked(
                            std::concat!($str, "\0").as_bytes()
                        )
                    },
                )*}
            }
        }
    };
}

layer_cstrs![KhronosValidation => "VK_LAYER_KHRONOS_validation",];
