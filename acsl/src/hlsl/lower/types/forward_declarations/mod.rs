use crate::program::{types::TypeManager, Type};

/// Produce forward declarations for types
pub(super) fn lower_type_forward_declarations(content: &mut String, types: &TypeManager) {
    for r#type in types {
        match &**r#type {
            Type::Primitive(_) | Type::Vector(_) | Type::Matrix(_) => {}
        }
    }
}
