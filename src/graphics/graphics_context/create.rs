use crate::{
    graphics::{GraphicsContext, Mesh, Shader, Vertex},
    Result,
};
use acsl::D3DProgram;
use std::{num::NonZeroU32, sync::atomic::Ordering};

impl GraphicsContext {
    /// Create a new [`Shader`]
    pub fn create_shader<V: Vertex>(&self, compiled_shader: &D3DProgram<V>) -> Result<Shader<V>> {
        let id = NonZeroU32::new(self.next_shader_id.fetch_add(1, Ordering::SeqCst)).unwrap();
        Shader::new(id, compiled_shader, &self.device)
    }

    /// Create a new [`Mesh`]
    pub fn create_mesh<V: Vertex>(&self, vertices: &[V], indices: &[u32]) -> Result<Mesh<V>> {
        Mesh::new(vertices, indices, &self.device)
    }

    /// Create a new [`Mesh`] without checking the elements
    pub unsafe fn create_mesh_unchecked<V: Vertex>(
        &self,
        vertices: &[V],
        indices: &[u32],
    ) -> Result<Mesh<V>> {
        Mesh::new_unchecked(vertices, indices, &self.device)
    }
}
