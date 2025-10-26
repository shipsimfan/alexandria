use crate::program::Type;

impl Type {
    /// Get the ID of this type
    pub fn id(&self) -> u32 {
        match self {
            Type::Primitive(primitive) => primitive.id(),
            Type::Vector(vector) => vector.id(),
            Type::Matrix(matrix) => matrix.id(),
            Type::Struct(r#struct) => r#struct.id(),
        }
    }

    /// Get the AST type ID of the node that defined this type
    pub fn ast_id(&self) -> Option<u32> {
        match self {
            Type::Primitive(_) => None,
            Type::Vector(_) => None,
            Type::Matrix(_) => None,
            Type::Struct(r#struct) => Some(r#struct.ast_id()),
        }
    }

    /// Set the type id reported
    pub(in crate::program::types) unsafe fn set_id(&mut self, id: u32) {
        match self {
            Type::Primitive(primitive) => primitive.set_id(id),
            Type::Vector(vector) => vector.set_id(id),
            Type::Matrix(matrix) => matrix.set_id(id),
            Type::Struct(r#struct) => r#struct.set_id(id),
        }
    }
}
