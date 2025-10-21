use crate::program::Type;

impl Type {
    /// Get the ID of this type
    pub fn id(&self) -> u32 {
        match self {
            Type::Primitive(primitive) => primitive.id(),
            Type::Vector(vector) => vector.id(),
            Type::Matrix(matrix) => matrix.id(),
        }
    }

    /// Set the type id reported
    pub(in crate::program::types) unsafe fn set_id(&mut self, id: u32) {
        match self {
            Type::Primitive(primitive) => primitive.set_id(id),
            Type::Vector(vector) => vector.set_id(id),
            Type::Matrix(matrix) => matrix.set_id(id),
        }
    }
}
