use crate::graphics::RenderFrame;
use win32::d3d11::ID3D11DeviceContext;

impl<'a> RenderFrame<'a> {
    /// Get the device context for rendering
    pub(in crate::graphics) fn device_context(&mut self) -> &mut ID3D11DeviceContext {
        &mut self.render_context.device_context
    }
}
