use crate::math::Vector2u;
#[cfg(debug_assertions)]
use info_queue::InfoQueue;
use swapchain_objects::SwapchainObjects;
use win32::{
    d3d11::ID3D11DeviceContext,
    dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_FLAG},
    ComPtr, UINT,
};

#[cfg(debug_assertions)]
mod info_queue;
mod render_frame;
mod swapchain_objects;

mod begin_render;
mod get_debug_messages;
mod new;
mod resize;

pub use render_frame::RenderFrame;

/// The context for rendering
pub struct RenderContext {
    /// The current size of the swapchain
    swapchain_size: Vector2u,

    /// The objects directly associated with the swapchain
    swapchain_objects: Option<SwapchainObjects>,

    /// The swap chain to render onto
    swapchain: ComPtr<IDXGISwapChain>,

    /// The device context for issuing rendering commands
    #[allow(unused)]
    device_context: ComPtr<ID3D11DeviceContext>,

    /// The info queue producing debug messages from the system graphics API
    #[cfg(debug_assertions)]
    info_queue: InfoQueue,
}

const SWAP_CHAIN_FLAGS: UINT = DXGI_SWAP_CHAIN_FLAG::AllowTearing as _;
