use crate::program::types::{Struct, StructField, StructFieldMeta};

/// Lower a struct definition
pub(super) fn lower(content: &mut String, r#struct: &Struct) {
    content.push_str("struct ");
    content.push_str(r#struct.hlsl_name());
    content.push_str(" {\n");

    for field in r#struct.fields() {
        lower_field(content, field);
    }

    content.push_str("}\n\n");
}

fn lower_field(content: &mut String, field: &StructField) {
    content.push_str("    ");
    content.push_str(field.r#type().hlsl_name());
    content.push(' ');
    content.push_str(field.name());

    if let Some(meta) = field.meta() {
        content.push_str(" : ");
        content.push_str(match meta {
            StructFieldMeta::Position => "SV_POSITION",
        });
    }

    content.push_str(";\n");
}
