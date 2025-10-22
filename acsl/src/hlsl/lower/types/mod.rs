use crate::program::types::TypeManager;
use definitions::lower_type_definitions;
use forward_declarations::lower_type_forward_declarations;

mod definitions;
mod forward_declarations;

/// Lower all types
pub(super) fn lower_types(content: &mut String, types: &TypeManager) {
    lower_type_forward_declarations(content, types);
    lower_type_definitions(content, types);
}
