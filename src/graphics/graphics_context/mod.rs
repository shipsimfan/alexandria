use win32::{d3d11::ID3D11Device, ComPtr};

mod get;
mod new;

/// The context for creating graphics objects
pub struct GraphicsContext {
    /// The device for creating render objects
    #[allow(unused)]
    device: ComPtr<ID3D11Device>,
}
