use crate::graphics::InputElementType;
use win32::dxgi::DXGI_FORMAT;

impl InputElementType {
    /// Convert this type into a D3D [`DXGI_FORMAT`]
    pub(in crate::graphics) fn to_d3d(&self) -> DXGI_FORMAT {
        match self {
            InputElementType::U32 => DXGI_FORMAT::R32UInt,
            InputElementType::F32 => DXGI_FORMAT::R32Float,
            InputElementType::Vector2F32 => DXGI_FORMAT::R32G32Float,
            InputElementType::Vector3F32 => DXGI_FORMAT::R32G32B32Float,
            InputElementType::Vector4F32 => DXGI_FORMAT::R32G32B32A32Float,
        }
    }
}
