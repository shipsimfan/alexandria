use crate::graphics::{Shader, Vertex};
use std::num::NonZeroU32;

impl<V: Vertex, CB> Shader<V, CB> {
    /// Get the ID assigned to this shader
    pub(in crate::graphics) fn id(&self) -> NonZeroU32 {
        self.id
    }
}
