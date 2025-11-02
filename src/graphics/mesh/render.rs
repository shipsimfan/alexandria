use crate::graphics::{Mesh, Vertex};
use win32::{d3d11::ID3D11DeviceContext, dxgi::DXGI_FORMAT};

impl<V: Vertex> Mesh<V> {
    /// Render this mesh on `device_context` using its current state
    pub(in crate::graphics) fn render(&mut self, device_context: &mut ID3D11DeviceContext) {
        let vertex_buffer = self.vertex_buffer.as_mut() as *mut _;
        let stride = std::mem::size_of::<V>() as _;
        let offset = 0;
        device_context.ia_set_vertex_buffers(0, 1, &vertex_buffer, &stride, &offset);
        device_context.ia_set_index_buffer(self.index_buffer.as_mut(), DXGI_FORMAT::R32UInt, 0);

        device_context.draw_indexed(self.index_count as _, 0, 0);
    }
}
