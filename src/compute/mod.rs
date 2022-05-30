use crate::{Input, LineMesh, Mesh, Window};

mod buffer;
mod shader;

pub use buffer::*;
pub use shader::*;

pub fn copy_compute_to_vertex<T, I: Input>(
    compute_buffer: &mut Buffer<T>,
    mesh: &mut Mesh<T>,
    window: &mut Window<I>,
) {
    window
        .device_context()
        .copy_resource(mesh.vertex_buffer(), compute_buffer.buffer());
}

pub fn copy_compute_to_vertex_line<T, I: Input>(
    compute_buffer: &mut Buffer<T>,
    mesh: &mut LineMesh<T>,
    window: &mut Window<I>,
) {
    window
        .device_context()
        .copy_resource(mesh.buffer(), compute_buffer.buffer());
}
