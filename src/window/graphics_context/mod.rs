use crate::math::Vector2u;
use win32::{
    d3d11::{ID3D11Device, ID3D11DeviceContext},
    dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_FLAG},
    ComPtr, UINT,
};

mod end_render;
mod new;
mod resize;

/// The context for rendering
pub struct GraphicsContext {
    /// The current size of the swapchain
    swap_chain_size: Vector2u,

    /// The swap chain to render onto
    swap_chain: ComPtr<IDXGISwapChain>,

    /// The device context for issuing rendering commands
    #[allow(unused)]
    device_context: ComPtr<ID3D11DeviceContext>,

    /// The device for creating render objects
    #[allow(unused)]
    device: ComPtr<ID3D11Device>,
}

const SWAP_CHAIN_FLAGS: UINT = DXGI_SWAP_CHAIN_FLAG::AllowTearing as _;
