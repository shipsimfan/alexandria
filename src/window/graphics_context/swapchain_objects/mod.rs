use win32::{
    d3d11::{ID3D11RenderTargetView, ID3D11Resource},
    ComPtr,
};

mod clear;
mod new;
mod unbind;

/// The objects associated directly with the swapchain that must be re-created when the swapchan is
/// resized
pub(in crate::window::graphics_context) struct SwapchainObjects {
    /// A reference to the swapchain's back buffer
    #[allow(unused)]
    back_buffer: ComPtr<ID3D11Resource>,

    /// The render target view of the swapchain's back buffer
    rtv: ComPtr<ID3D11RenderTargetView>,
}
