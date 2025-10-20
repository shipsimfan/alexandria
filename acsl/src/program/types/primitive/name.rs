use crate::program::types::Primitive;

impl Primitive {
    /// Gets the name of this primitive type
    pub fn name(&self) -> &str {
        match self {
            Primitive::F32 => "f32",
            Primitive::U32 => "u32",
        }
    }
}
