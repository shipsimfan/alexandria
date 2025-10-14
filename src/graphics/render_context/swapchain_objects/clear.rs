use crate::graphics::render_context::SwapchainObjects;
use win32::d3d11::ID3D11DeviceContext;

impl SwapchainObjects {
    /// Clear the swapchain's back buffer
    pub fn clear(&mut self, device_context: &mut ID3D11DeviceContext, color: [f32; 4]) {
        device_context.clear_render_target_view(self.rtv.as_mut(), color.as_ptr())
    }
}
