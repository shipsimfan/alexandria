use crate::graphics::{Shader, Vertex};
use std::num::NonZeroU32;

impl<V: Vertex> Shader<V> {
    /// Get the ID assigned to this shader
    pub(in crate::graphics) fn id(&self) -> NonZeroU32 {
        self.id
    }
}
