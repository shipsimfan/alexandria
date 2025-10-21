use crate::program::Type;

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(primitive) => primitive.fmt(f),
            Type::Vector(vector) => vector.fmt(f),
            Type::Matrix(matrix) => matrix.fmt(f),
        }
    }
}
