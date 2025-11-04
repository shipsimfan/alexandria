use crate::graphics::{Mesh, RenderFrame, Shader, Vertex};

impl<'a> RenderFrame<'a> {
    /// Render `mesh` using `shader`
    pub fn render<V: Vertex, CB>(&mut self, mesh: &mut Mesh<V>, shader: &mut Shader<V, CB>) {
        self.set_active_shader(shader);
        mesh.render(&mut self.render_context.device_context);
    }
}
