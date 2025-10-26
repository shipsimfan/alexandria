use crate::program::Type;

impl Type {
    /// Get the name of this type
    pub fn name(&self) -> &str {
        match self {
            Type::Primitive(primitive) => primitive.name(),
            Type::Vector(vector) => vector.name(),
            Type::Matrix(matrix) => matrix.name(),
            Type::Struct(r#struct) => r#struct.name(),
        }
    }

    #[cfg(feature = "hlsl")]
    pub fn hlsl_name(&self) -> &str {
        match self {
            Type::Primitive(primitive) => primitive.hlsl_name(),
            Type::Vector(vector) => vector.hlsl_name(),
            Type::Matrix(matrix) => matrix.hlsl_name(),
            Type::Struct(r#struct) => r#struct.hlsl_name(),
        }
    }
}
