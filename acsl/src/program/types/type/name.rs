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
}
