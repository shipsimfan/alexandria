use win32::{
    d3d11::{ID3D11Device, ID3D11DeviceContext},
    dxgi::IDXGISwapChain,
    ComPtr,
};

mod end_render;
mod new;
mod resize;

/// The context for rendering
pub struct GraphicsContext {
    /// Should presents be synchronized with vertical blanks?
    vsync: bool,

    /// The swap chain to render onto
    swap_chain: ComPtr<IDXGISwapChain>,

    /// The device context for issuing rendering commands
    #[allow(unused)]
    device_context: ComPtr<ID3D11DeviceContext>,

    /// The device for creating render objects
    #[allow(unused)]
    device: ComPtr<ID3D11Device>,
}
