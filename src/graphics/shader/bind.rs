use crate::graphics::{Shader, Vertex};
use std::ptr::null;
use win32::d3d11::ID3D11DeviceContext;

impl<V: Vertex, CB> Shader<V, CB> {
    /// Set this shader as the active shader for rendering
    pub(in crate::graphics) fn bind(&mut self, device_context: &mut ID3D11DeviceContext) {
        device_context.vs_set_shader(self.vertex_shader.as_mut(), null(), 0);
        device_context.ps_set_shader(self.pixel_shader.as_mut(), null(), 0);
        device_context.ia_set_input_layout(self.input_layout.as_mut());

        if let Some(constant_buffer) = &mut self.constant_buffer {
            let constant_buffer = constant_buffer.as_mut() as *mut _;
            device_context.vs_set_constant_buffers(0, 1, &constant_buffer);
        }
    }
}
