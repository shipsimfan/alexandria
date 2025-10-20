use crate::program::{
    types::{Matrix, Primitive, Vector},
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
