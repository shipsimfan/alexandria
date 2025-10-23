use crate::program::{types::TypeManager, Type};

/// Produce type definitions
pub(super) fn lower_type_definitions(content: &mut String, types: &TypeManager) {
    for r#type in types {
        match &**r#type {
            Type::Primitive(_) | Type::Vector(_) | Type::Matrix(_) => {}
            Type::Struct(_) => todo!(),
        }
    }
}
