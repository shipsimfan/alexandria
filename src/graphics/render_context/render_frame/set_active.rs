use crate::graphics::{RenderFrame, Shader, Vertex};

impl<'a> RenderFrame<'a> {
    pub(in crate::graphics) fn set_active_shader<V: Vertex>(&mut self, shader: &mut Shader<V>) {
        if self.render_context.current_shader == Some(shader.id()) {
            return;
        }

        shader.set_active(&mut self.render_context.device_context);
        self.render_context.current_shader = Some(shader.id());
    }
}
