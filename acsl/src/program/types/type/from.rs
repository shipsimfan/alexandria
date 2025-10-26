use crate::program::{
    types::{Matrix, Primitive, Struct, Vector},
    Type,
};

impl From<Primitive> for Type {
    fn from(primitive: Primitive) -> Self {
        Type::Primitive(primitive)
    }
}

impl From<Vector> for Type {
    fn from(vector: Vector) -> Self {
        Type::Vector(vector)
    }
}

impl From<Matrix> for Type {
    fn from(matrix: Matrix) -> Self {
        Type::Matrix(matrix)
    }
}

impl From<Struct> for Type {
    fn from(r#struct: Struct) -> Self {
        Type::Struct(r#struct)
    }
}
