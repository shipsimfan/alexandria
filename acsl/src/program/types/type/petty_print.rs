use crate::{
    pretty_print::{PrettyFormatter, PrettyPrint},
    program::Type,
};

impl PrettyPrint for Type {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        match self {
            Type::Primitive(primitive) => primitive.fmt(depth, f),
            Type::Vector(vector) => vector.fmt(depth, f),
            Type::Matrix(matrix) => matrix.fmt(depth, f),
            Type::Struct(r#struct) => r#struct.fmt(depth, f),
        }
    }
}
