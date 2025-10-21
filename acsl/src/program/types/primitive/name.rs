use crate::program::types::Primitive;

impl Primitive {
    /// Gets the name of this primitive type
    pub const fn name(&self) -> &'static str {
        match self {
            Primitive::F32 => "f32",
            Primitive::U32 => "u32",
        }
    }
}
