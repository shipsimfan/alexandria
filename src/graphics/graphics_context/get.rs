use crate::graphics::GraphicsContext;
use win32::d3d11::ID3D11Device;

impl GraphicsContext {
    /// Get the device used to create graphics objects
    pub(in crate::graphics) fn device(&self) -> &ID3D11Device {
        &self.device
    }
}
