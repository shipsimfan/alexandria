use crate::{window::graphics_context::SwapchainObjects, Result, FORMAT};
use win32::{
    d3d11::{
        ID3D11Device, ID3D11Resource, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION,
        D3D11_TEX2D_RTV,
    },
    dxgi::IDXGISwapChain,
    try_hresult, ComInterface, ComPtr,
};

impl SwapchainObjects {
    /// Create new [`SwapchainObjects`]
    pub fn new(swapchain: &mut IDXGISwapChain, device: &ID3D11Device) -> Result<Self> {
        let mut back_buffer = ComPtr::<ID3D11Resource>::new_in(|ptr| {
            try_hresult!(swapchain.get_buffer(0, &ID3D11Resource::IID, ptr.cast()))
        })?;

        let rtv = ComPtr::new_in(|ptr| {
            try_hresult!(device.create_render_target_view(
                back_buffer.as_mut(),
                &D3D11_RENDER_TARGET_VIEW_DESC {
                    format: FORMAT,
                    view_dimension: D3D11_RTV_DIMENSION::Texture2D,
                    u: win32::d3d11::D3D11_RENDER_TARGET_VIEW_DESC_UNION {
                        texture_2d: D3D11_TEX2D_RTV { mip_slice: 0 }
                    }
                },
                ptr
            ))
        })?;

        Ok(SwapchainObjects { back_buffer, rtv })
    }
}
