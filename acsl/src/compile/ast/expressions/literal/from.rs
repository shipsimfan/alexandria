use crate::compile::{
    ast::expressions::{Literal, LiteralKind},
    tokens::{FloatingPoint, Integer},
};

impl From<FloatingPoint> for Literal {
    fn from(value: FloatingPoint) -> Self {
        Literal {
            kind: LiteralKind::FloatingPoint(value.value()),
            span: value.span(),
        }
    }
}

impl From<Integer> for Literal {
    fn from(value: Integer) -> Self {
        Literal {
            kind: LiteralKind::Integer(value.value()),
            span: value.span(),
        }
    }
}
