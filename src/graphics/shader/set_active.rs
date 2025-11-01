use crate::graphics::{Shader, Vertex};
use std::ptr::null;
use win32::d3d11::ID3D11DeviceContext;

impl<V: Vertex> Shader<V> {
    /// Set this shader as the active shader for rendering
    pub(in crate::graphics) fn set_active(&mut self, device_context: &mut ID3D11DeviceContext) {
        device_context.vs_set_shader(self.vertex_shader.as_mut(), null(), 0);
        device_context.ps_set_shader(self.pixel_shader.as_mut(), null(), 0);
        device_context.ia_set_input_layout(self.input_layout.as_mut());
    }
}
