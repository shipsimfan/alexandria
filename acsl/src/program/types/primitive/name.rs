use crate::program::types::Primitive;

impl Primitive {
    /// Gets the name of this primitive type
    pub const fn name(&self) -> &'static str {
        match self {
            Primitive::Unit => "()",
            Primitive::F32 => "f32",
            Primitive::U32 => "u32",
        }
    }

    /// Gets the name of this primitive type in HLSL
    #[cfg(feature = "hlsl")]
    pub const fn hlsl_name(&self) -> &'static str {
        match self {
            Primitive::Unit => "void",
            Primitive::F32 => "float",
            Primitive::U32 => "uint",
        }
    }
}
