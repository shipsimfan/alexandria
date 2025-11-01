use crate::{
    graphics::{GraphicsContext, Shader, Vertex},
    Result,
};
use acsl::D3DProgram;
use std::num::NonZeroU32;

impl GraphicsContext {
    /// Create a new [`Shader`]
    pub fn create_shader<V: Vertex>(
        &mut self,
        compiled_shader: &D3DProgram<V>,
    ) -> Result<Shader<V>> {
        let id = self.next_shader_id;
        self.next_shader_id = unsafe { NonZeroU32::new_unchecked(self.next_shader_id.get() + 1) };
        Shader::new(id, compiled_shader, &self.device)
    }
}
