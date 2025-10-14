use crate::graphics::GraphicsContext;
use win32::{d3d11::ID3D11Device, ComPtr};

impl GraphicsContext {
    /// Creates a new [`GraphicsContext`]
    pub(in crate::graphics) fn new(device: ComPtr<ID3D11Device>) -> Self {
        GraphicsContext { device }
    }
}
