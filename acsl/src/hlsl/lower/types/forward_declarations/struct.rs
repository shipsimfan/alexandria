use crate::program::types::Struct;

/// Lower a struct declaration
pub(super) fn lower(content: &mut String, r#struct: &Struct) {
    content.push_str("struct ");
    content.push_str(r#struct.name());
    content.push_str(";\n");
}
