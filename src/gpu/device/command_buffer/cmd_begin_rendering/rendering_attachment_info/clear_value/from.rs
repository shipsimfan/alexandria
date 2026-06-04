use crate::{
    gpu::VulkanClearValue,
    math::{Color4, ColorSpace},
};

impl<Space: ColorSpace<f32>> From<Color4<f32, Space>> for VulkanClearValue {
    fn from(color: Color4<f32, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorF32(color.into())
    }
}

impl<Space: ColorSpace<i32>> From<Color4<i32, Space>> for VulkanClearValue {
    fn from(color: Color4<i32, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorI32(color.into())
    }
}

impl<Space: ColorSpace<i16>> From<Color4<i16, Space>> for VulkanClearValue {
    fn from(color: Color4<i16, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorI32([color.r as _, color.g as _, color.b as _, color.a as _])
    }
}

impl<Space: ColorSpace<i8>> From<Color4<i8, Space>> for VulkanClearValue {
    fn from(color: Color4<i8, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorI32([color.r as _, color.g as _, color.b as _, color.a as _])
    }
}

impl<Space: ColorSpace<u32>> From<Color4<u32, Space>> for VulkanClearValue {
    fn from(color: Color4<u32, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorU32(color.into())
    }
}

impl<Space: ColorSpace<u16>> From<Color4<u16, Space>> for VulkanClearValue {
    fn from(color: Color4<u16, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorU32([color.r as _, color.g as _, color.b as _, color.a as _])
    }
}

impl<Space: ColorSpace<u8>> From<Color4<u8, Space>> for VulkanClearValue {
    fn from(color: Color4<u8, Space>) -> VulkanClearValue {
        VulkanClearValue::ColorU32([color.r as _, color.g as _, color.b as _, color.a as _])
    }
}

impl From<(f32, u32)> for VulkanClearValue {
    fn from((depth, stencil): (f32, u32)) -> Self {
        VulkanClearValue::DepthStencil { depth, stencil }
    }
}

impl From<(f32, u16)> for VulkanClearValue {
    fn from((depth, stencil): (f32, u16)) -> Self {
        VulkanClearValue::DepthStencil {
            depth,
            stencil: stencil as _,
        }
    }
}

impl From<(f32, u8)> for VulkanClearValue {
    fn from((depth, stencil): (f32, u8)) -> Self {
        VulkanClearValue::DepthStencil {
            depth,
            stencil: stencil as _,
        }
    }
}
